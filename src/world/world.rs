use std::cell::Cell;
use glam::{Affine3A, Vec3A, Vec2};

use crate::render_registry::pipelines::PipelineLabel;
use super::{color::Color, material::Material};
use crate::render_registry::alloc::{Position,  BuffersAllocPosition};
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
                $attr: Registry<$prim_ty>,
            )*
            insert_order: Vec<Ref<Global>>,
            materials: Vec<Box<dyn Material>>,
        }
        impl World {
            pub fn new() -> Self {
                Self {
                    $(
                        $attr: Registry::new(),
                    )*
                    insert_order: Vec::new(),
                    materials: Vec::new(),
                }
            }
            pub fn update(&self, ctx: WorldUpdateCtx) {
                for &Ref {index, label} in &self.insert_order {
                    match label {
                        $(
                            Global::$prim_ty => unsafe {self.$attr.update(index, ctx.var_update, self)},
                        )*
                    }
                }
                for (i, mat) in self.materials.iter().enumerate() {
                    let pos = &ctx.allocs[i];
                    let vertex = &mut ctx.views[pos.pipe_label as usize][pos.vertex_bound.clone()];
                    mat.put(
                        ctx.var_update,
                        self,
                        vertex,
                        pos.index_bound.start as u32,
                        &mut [0; 100],
                    )
                }
            }
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
    composite:
    - Point = Vec3A, Affine3A into ;
);

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

pub struct Registry<T> {
    // TODO: replace unaf
    vars: Vec<(Cell<T>, Box<dyn Variator<Item=T>>)>,
}
impl<T: Copy + Default> Registry<T> {
    pub fn new() -> Self {
        Self {
            vars: Vec::new()
        }
    }
    // safety: this ref should be the only one at that time
    unsafe fn update(&self, idx: usize, ctx: UpdateCtx, world: &World) {
        let (cell, var) = &self.vars[idx];
        // if is_borrow.get() { panic!("Unexpected update of a variator while updating it"); }
        // is_borrow.set(true);
        // // SAFETY:
        // // - There should be no other read ref at this time
        // // - UnsafeCell ensure that the ref is valid
        // let prev = unsafe { &mut *prev.get() };
        cell.set(var.update(ctx, world));
        // is_borrow.set(false);
    }
    fn get(&self, idx: usize) -> T {
        // let (ptr, is_borrow, _) = &self.vars[idx];
        // if is_borrow.get() {
        //     panic!("Unexpected access of a variator cache while updating it");
        // }
        // // SAFETY:
        // // - no mutable ref exists, as is_borrow is false
        // // - UnsafeCell ensure that the ref is valid
        // unsafe { (&*ptr.get()).clone() }
        self.vars[idx].0.get()
    }
    fn push(&mut self, var: impl Variator<Item=T>+'static) -> usize {
        let idx = self.vars.len();
        self.vars.push((Cell::new(T::default()), Box::new(var)));
        idx
    }
}

pub struct WorldUpdateCtx<'a> {
    pub var_update: UpdateCtx,
    pub views: [&'a mut [u32]; PipelineLabel::COUNT],
    pub allocs: &'a [Position],
}

impl Variator for Ref<Point> {
    type Item = Vec3A;
    fn update(&self, _ctx: UpdateCtx, world: &World) -> Self::Item {
        match self.label {
            Point::Vec3A => world.vec3a.get(self.index),
            Point::Affine3A => world.vec3a.get(self.index),
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
