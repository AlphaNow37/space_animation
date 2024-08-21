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
            $attr: ident : $prim_ty: ident : $prim_pty: ident
        );*
        $(;)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Global {
            $($prim_pty,)*
        }
        $(
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct $prim_pty;
            impl Into<Global> for $prim_pty {
                fn into(self) -> Global {
                    Global::$prim_pty
                }
            }
            impl PrimPush for $prim_ty {
                type Label = $prim_pty;
                fn push(world: &mut World, var: impl Variator<Item=Self>+'static) -> Ref<Self::Label> {
                    Ref {
                        index: world.$attr.push(var),
                        label: $prim_pty,
                    }
                }
            }
            impl Variator for $prim_ty {
                type Item=$prim_ty;
                fn update(&self, _ctx: UpdateCtx, _world: &World) -> $prim_ty {
                    *self
                }
            }
            impl Variator for Ref<$prim_pty> {
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
            insert_order: Vec<Ref<Global>>,
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
                for &Ref {index, label} in &self.insert_order {
                    match label {
                        $(
                            Global::$prim_pty => self.$attr.update(index, ctx, self),
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
    - vec3: Vec3: PVec3;
    - transform: Transform: PTransform;
    - color: Color: PColor;
    - f32: F32: PF32;
    - vec2: Vec2: PVec2;
    - camera: Camera: PCamera;
    - angle: Angle: PAngle;
    - dir: Dir: PDir;
    - vec4: Vec4: PVec4;
);

impl World {
    pub fn push<T: Push + 'static>(&mut self, var: T) -> Ref<T::Label> {
        let id = var.push(self);
        self.insert_order.push(id.as_ref());
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
    label: T,
    index: usize,
}
impl<T: Copy> Ref<T> {
    pub fn as_ref<U>(&self) -> Ref<U>
    where
        T: Into<U>,
    {
        Ref {
            index: self.index,
            label: self.label.into(),
        }
    }
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

pub trait Push {
    type Label: Into<Global> + Copy;
    fn push(self, world: &mut World) -> Ref<Self::Label>;
}
pub trait PrimPush {
    type Label: Into<Global> + Copy;
    fn push(world: &mut World, var: impl Variator<Item = Self> + 'static) -> Ref<Self::Label>;
}
impl<T: Variator + 'static> Push for T
where
    T::Item: PrimPush,
{
    type Label = <T::Item as PrimPush>::Label;
    fn push(self, world: &mut World) -> Ref<Self::Label> {
        <T::Item as PrimPush>::push(world, self)
    }
}
