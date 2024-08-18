use glam::{Affine3A, Vec3A, Vec2, Mat3A};

use crate::render_registry::pipelines::PipelineLabel;
use crate::render_registry::alloc::{Position,  BuffersAllocPosition};

use super::{color::Color, material::Material, camera::Camera, register::Register, rotation::Angle};
use super::variator::{UpdateCtx, Variator};

macro_rules! make_system {
    (
        primitive: $(
            - 
            $attr: ident : $prim_ty: ident : $prim_pty: ident
            into $($prim_into: ident),*
        );*
        $(;)?
        composite: $(
            - $compo_ty: ident = $($sub: ident),*
            into $($compo_into: ident),*
        );*
        $(;)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Global {
            $($prim_ty,)*
        }
        $(
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct $prim_pty;
            impl Into<Global> for $prim_pty {
                fn into(self) -> Global {
                    Global::$prim_ty
                }
            }
            $(
                impl Into<$prim_into> for $prim_pty {
                    fn into(self) -> $prim_into {
                        $prim_into::$prim_ty
                    }
                }
            )*
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
        $(
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum $compo_ty {
                $(
                    $sub,
                )*
            }
            impl Into<Global> for $compo_ty {
                fn into(self) -> Global {
                    match self {
                        $(
                            $compo_ty::$sub => Global::$sub,
                        )*
                    }
                }
            }
            impl TryFrom<Global> for $compo_ty {
                type Error = ();
                fn try_from(val: Global) -> Result<$compo_ty, ()> {
                    Ok(match val {
                        $(
                            Global::$sub => $compo_ty::$sub,
                        )*
                        _ => {return Err(());}
                    })
                }
            }
            $(
                impl Into<$compo_into> for $compo_ty {
                    fn into(&self) -> Ref<$compo_into> {
                        let glob: Global = self.into();
                        glob.try_into().unwrap()
                    }
                }
            )*
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
    - vec3a: Vec3A: PVec3A into Point;
    - affine3a: Affine3A: PAffine3A into Point;
    - color: Color: PColor into ;
    - f32: F32: PF32 into ;
    - vec2: Vec2: PVec2 into ;
    - camera: Camera: PCamera into Point;
    - angle: Angle: PAngle into ;
    - mat3a: Mat3A: PMat3a into ;
    composite:
    - Point = Vec3A, Affine3A, Camera into ;
);

impl World {
    pub fn push<T: Push+'static>(&mut self, var: T) -> Ref<T::Label> {
        let id = var.push(self);
        self.insert_order.push(id.as_ref());
        id
    }
    pub fn allocs(&self, alloc: &mut BuffersAllocPosition) -> Vec<Position> {
        self.materials.iter()
            .map(|m| m.alloc(alloc))
            .collect()
    }
    pub fn push_mat(&mut self, mat: impl Material+'static) {
        self.materials.push(Box::new(mat));
    }
    fn redraw(&self, ctx: &mut WorldUpdateCtx) {
        for (i, mat) in self.materials.iter().enumerate() {
            let pos = &ctx.allocs[i];
            let (glob_vertex, glob_index) = &mut ctx.views[pos.pipe_label as usize];
            mat.put(
                ctx.var_update,
                self,
                &mut glob_vertex[pos.vertex_bound.clone()],
                pos.vertex_bound.start as u32,
                &mut &mut glob_index[pos.index_bound.clone()]
            )
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
impl<T> Ref<T> {
    pub fn as_ref<U>(self) -> Ref<U> where T: Into<U> {
        Ref {
            index: self.index,
            label: self.label.into()
        }
    }
}

pub struct WorldUpdateCtx<'a> {
    pub var_update: UpdateCtx,
    pub views: [(&'a mut [u8], &'a mut [u8]); PipelineLabel::COUNT],
    pub allocs: &'a [Position],
    pub cam: Camera,
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
}

impl Variator for Ref<Point> {
    type Item = Vec3A;
    fn update(&self, _ctx: UpdateCtx, world: &World) -> Self::Item {
        match self.label {
            Point::Vec3A => world.vec3a.get(self.index),
            Point::Affine3A => world.affine3a.get(self.index).translation,
            Point::Camera => world.camera.get(self.index).pos.translation,
        }
    }
}

pub trait Push {
    type Label: Into<Global>+Copy;
    fn push(self, world: &mut World) -> Ref<Self::Label>;
}
pub trait PrimPush {
    type Label: Into<Global>+Copy;
    fn push(world: &mut World, var: impl Variator<Item=Self>+'static) -> Ref<Self::Label>;
}
impl<T: Variator+'static> Push for T where T::Item: PrimPush {
    type Label=<T::Item as PrimPush>::Label;
    fn push(self, world: &mut World) -> Ref<Self::Label> {
        <T::Item as PrimPush>::push(world, self)
    }
}
