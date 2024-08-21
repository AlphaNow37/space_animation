use std::marker::PhantomData;
use crate::math::{Vec2, Vec3, Vec4, Transform, Dir};
use crate::render_registry::alloc::{BuffersAllocPosition};
use crate::world::primitives::camera::Camera;
use crate::math::Angle;
use crate::render_registry::mesh_builder::MeshBuilders;
use crate::world::visuals::material::Material;

use super::variators::variator::{UpdateCtx, Variator};
use super::{
    primitives::color::Color, register::Register,
};

macro_rules! make_system {
    (
        primitive: $(
            -
            $attr: ident : $prim_ty: ident
        );*
        $(;)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Global {
            $($prim_ty,)*
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct GlobalRef {
            index: usize,
            label: Global,
        }
        $(
            impl PrimPush for $prim_ty {
                fn push(world: &mut World, var: impl Variator<Item=Self>+'static) -> Ref<Self> {
                    Ref {
                        index: world.$attr.push(var),
                        label: PhantomData,
                    }
                }
                fn global_label() -> Global {
                    Global::$prim_ty
                }
            }
            impl Variator for $prim_ty {
                type Item=$prim_ty;
                fn update(&self, _ctx: UpdateCtx, _world: &World) -> $prim_ty {
                    *self
                }
            }
            impl Variator for Ref<$prim_ty> {
                type Item=$prim_ty;
                fn update(&self, _ctx: UpdateCtx, world: &World) -> $prim_ty {
                    world.$attr.get(self.index)
                }
            }
        )*
        pub struct World {
            $(
                $attr: Register<$prim_ty>,
            )*
            insert_order: Vec<GlobalRef>,
            materials: Vec<Box<dyn Material>>,
            pub settings: WorldSettings,
        }
        impl World {
            pub fn new() -> Self {
                $(
                    let $attr = Register::new();
                )*
                Self {
                    $($attr,)*
                    insert_order: Vec::new(),
                    materials: Vec::new(),
                    settings: WorldSettings::default(),
                }
            }
            fn update_registers(&self, ctx: UpdateCtx) {
                for &GlobalRef {index, label} in &self.insert_order {
                    match label {
                        $(
                            Global::$prim_ty => self.$attr.update(index, ctx, self),
                        )*
                    }
                }
            }
        }
    };
}
type F32 = f32;
make_system!(
    primitive:
    - vec3: Vec3;
    - transform: Transform;
    - color: Color;
    - f32: F32;
    - vec2: Vec2;
    - camera: Camera;
    - angle: Angle;
    - dir: Dir;
    - vec4: Vec4;
);

impl World {
    pub fn push<T: Variator + 'static>(&mut self, var: T) -> Ref<T::Item> where T::Item: PrimPush {
        let id = T::Item::push(self, var);
        self.insert_order.push(GlobalRef {
            label: T::Item::global_label(),
            index: id.index,
        });
        id
    }
    pub fn allocs(&self, alloc: &mut BuffersAllocPosition) {
        for mat in &self.materials {
            mat.alloc(alloc)
        }
    }
    pub fn push_mat(&mut self, mat: impl Material + 'static) {
        self.materials.push(Box::new(mat));
    }
    fn redraw(&self, ctx: &mut WorldUpdateCtx) {
        for mat in &self.materials {
            mat.put(
                ctx.var_update,
                self,
                &mut ctx.builders,
            );
        }
    }
    fn update_settings(&mut self, ctx: &WorldUpdateCtx) {
        self.settings = WorldSettings {
            cam_settings: ctx.cam,
        }
    }
    pub fn update(&mut self, mut ctx: WorldUpdateCtx) {
        self.update_settings(&ctx);
        self.update_registers(ctx.var_update);
        self.redraw(&mut ctx);
    }
    pub fn get_cam(&self, idx: isize) -> Camera {
        self.camera.get_mod(idx)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ref<T> {
    label: PhantomData<fn()->T>,
    index: usize,
}

pub struct WorldUpdateCtx<'a> {
    pub var_update: UpdateCtx,
    pub builders: MeshBuilders<'a>,
    pub cam: Camera,
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
}

pub trait PrimPush: Sized {
    fn push(world: &mut World, var: impl Variator<Item = Self> + 'static) -> Ref<Self>;
    fn global_label() -> Global;
}
