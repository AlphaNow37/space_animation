use std::marker::PhantomData;
use crate::math::{Vec2, Vec3, Vec4, Transform, Dir, Polynomial};
use crate::render_registry::alloc::{BufferAllocator};
use crate::world::primitives::camera::Camera;
use crate::math::Angle;
use crate::render_registry::mesh_builder::MeshBuilders;
use crate::world::stores::StoreLabel;
use crate::world::visuals::material::Material;

use super::variators::variator::{UpdateCtx, Variator};
use super::{
    primitives::color::Color, register::Register,
};
macro_rules! make_system {
    (
        $(
            $attr: ident : $prim_ty: ident $(($store_method: ident, $len_method: ident))?
        );*
        $(;)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum GlobalLabel {
            $($prim_ty),*
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct GlobalRef {
            index: usize,
            label: GlobalLabel,
        }
            $(
                impl PrimPush for $prim_ty {
                    fn push(world: &mut World, var: impl Variator<Item=Self>) -> usize {
                        world.$attr.push(var)
                    }
                    fn global_label() -> GlobalLabel {
                        GlobalLabel::$prim_ty
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
                Self {
                    $(
                        $attr: Register::new(),
                    )*
                    insert_order: Vec::new(),
                    materials: Vec::new(),
                    settings: WorldSettings::default(),
                }
            }
            fn update_registers(&self, ctx: UpdateCtx) {
                for &GlobalRef {index, label} in &self.insert_order {
                    match label {
                        $(
                            GlobalLabel::$prim_ty => self.$attr.update(index, ctx, self),
                        )*
                    }
                }
            }
            $($(
                pub fn $store_method(&self, buf: &mut [u32]) {
                    self.$attr.write(buf)
                }
                pub fn $len_method(&self) -> usize {
                    self.$attr.len()
                }
            )?)*
        }
    };
}

type F32 = f32;
type Color2 = (Color, Color);
type Polynomial4x4 = Polynomial<Vec3, 4, 4>;
make_system!(
    vec3: Vec3 (store_vec3, len_vec3);
    transform: Transform (store_transform, len_transform);
    color: Color (store_color, len_color);
    f32: F32 (store_f32, len_f32);
    vec2: Vec2 (store_vec2, len_vec2);
    camera: Camera;
    angle: Angle;
    dir: Dir;
    vec4: Vec4 (store_vec4, len_vec4);
    color2: Color2 (store_color2, len_color2);
    polynomial4x4: Polynomial4x4 (store_poly4x4, len_poly4x4);
);

impl World {
    pub fn allocs(&self, alloc: &mut BufferAllocator) {
        for mat in &self.materials {
            mat.alloc(alloc)
        }
        for label in StoreLabel::ARRAY {
            label.alloc(self, alloc);
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
        for label in StoreLabel::ARRAY {
            label.write(ctx.stores[label as usize], self);
        }
        self.redraw(&mut ctx);
    }
    pub fn get_cam(&self, idx: isize) -> Camera {
        self.camera.get_mod(idx)
    }
    pub fn push<T: Variator>(&mut self, var: T) -> Ref<T::Item> where T::Item: PrimPush {
        let idx = T::Item::push(self, var);
        self.insert_order.push(GlobalRef {
            label: T::Item::global_label(),
            index: idx,
        });
        Ref {
            index: idx,
            label: PhantomData,
        }
    }
}

pub type Main<T> = T;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ref<T> {
    label: PhantomData<fn()->T>,
    index: usize,
}
impl<T> Ref<T> {
    pub fn index(&self) -> usize {self.index}
}

pub struct WorldUpdateCtx<'a> {
    pub var_update: UpdateCtx,
    pub builders: MeshBuilders<'a>,
    pub stores: [&'a mut [u32]; StoreLabel::COUNT],
    pub cam: Camera,
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
}

pub trait PrimPush: Sized + 'static {
    fn push(world: &mut World, var: impl Variator<Item=Self>) -> usize;
    fn global_label() -> GlobalLabel;
}
