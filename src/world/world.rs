use std::marker::PhantomData;
use crate::math::{Vec2, Vec3, Vec4, Transform, Dir, Polynomial};
use crate::render_registry::alloc::{BufferAllocator};
use crate::world::primitives::camera::Camera;
use crate::math::Angle;
use crate::render_registry::mesh_builder::MeshBuilders;
use crate::render_registry::storage_structs::AsStrorageStruct;
use crate::utils::array_key;
use crate::world::visuals::material::Material;

use super::variators::variator::{UpdateCtx, Variator};
use super::{
    primitives::color::Color, register::Register,
};
macro_rules! make_system {
    (
        $(
            $reg: ident ($global_attr: ident) ($world_reg: ident) ($holder: ident $($getattr: tt)?) ($push_trait: ident $push_method: ident):
            $(
                - $attr: ident : $prim_ty: ident
            );*
        );*
        $(;)?
    ) => {

        $(
            array_key!(
                pub enum $global_attr {
                    $(
                        $prim_ty,
                    )*
                }
            );
        )*

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum GlobalLabel {
            $(
                $holder($global_attr),
            )*
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct GlobalRef {
            index: usize,
            label: GlobalLabel,
        }
        $(
            pub trait $push_trait: Sized {
                fn push(world: &mut World, var: impl Variator<Item=Self>) -> usize;
                fn global_label() -> GlobalLabel;
                fn _holder_to_inner<T>(holder: $holder<T>) -> T {
                    holder $(. $getattr)?
                }
            }
            impl World {
                pub fn $push_method<T: Variator>(&mut self, var: T) -> Ref<$holder<T::Item>> where T::Item: $push_trait  {
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
            $(
                impl $push_trait for $prim_ty {
                    fn push(world: &mut World, var: impl Variator<Item=Self>) -> usize {
                        world.$reg.$attr.push(var)
                    }
                    fn global_label() -> GlobalLabel {
                        GlobalLabel::$holder($global_attr::$prim_ty)
                    }
                }
                impl Variator for $holder<$prim_ty> {
                    type Item=$prim_ty;
                    fn update(&self, _ctx: UpdateCtx, _world: &World) -> $prim_ty {
                        <$prim_ty as $push_trait>::_holder_to_inner(*self)
                    }
                }
                impl Variator for Ref<$holder<$prim_ty>> {
                    type Item=$prim_ty;
                    fn update(&self, _ctx: UpdateCtx, world: &World) -> $prim_ty {
                        world.$reg.$attr.get(self.index)
                    }
                }
            )*
        )*

        $(
            struct $world_reg {
                $(
                    $attr: Register<$prim_ty>,
                )*
            }
        )*

        pub struct World {
            $(
                $reg: $world_reg,
            )*
            insert_order: Vec<GlobalRef>,
            materials: Vec<Box<dyn Material>>,
            pub settings: WorldSettings,
        }
        impl World {
            pub fn new() -> Self {
                Self {
                    $(
                        $reg: $world_reg {
                            $(
                                $attr: Register::new(),
                            )*
                        },
                    )*
                    insert_order: Vec::new(),
                    materials: Vec::new(),
                    settings: WorldSettings::default(),
                }
            }
            fn update_registers(&self, ctx: UpdateCtx) {
                for &GlobalRef {index, label} in &self.insert_order {
                    match label {
                        $($(
                            GlobalLabel::$holder($global_attr::$prim_ty) => self.$reg.$attr.update(index, ctx, self),
                        )*)*
                    }
                }
            }

        }
    };
}

macro_rules! impl_write_store {
    (
        $(
            $idx: literal => $attr: ident $ty: ident ($($stage: ident),* $(,)?)
        ),* $(,)?
    ) => {
        impl World {
            pub fn store_allocs(&self, alloc: &mut BufferAllocator) {
                $(
                    alloc.alloc_store($idx, self.store.$attr.len());
                )*
            }
            pub fn store_update(&self, stores: &mut [&mut [u32]; GlobalStore::COUNT]) {
                $(
                    self.store.$attr.write(stores[$idx]);
                )*
            }
            // TODO: move this out of world.rs
            pub fn store_stages() -> [wgpu::ShaderStages; GlobalStore::COUNT] {
                [$(
                    wgpu::ShaderStages::NONE $(| wgpu::ShaderStages::$stage)*,
                )*]
            }
            pub fn store_name() -> [&'static str; GlobalStore::COUNT] {
                [$(
                    concat!("Store ", stringify!($idx), ": ", stringify!($attr)),
                )*]
            }
            pub fn store_sized() -> [usize; GlobalStore::COUNT] {
                [$(
                    std::mem::size_of::<<$ty as AsStrorageStruct>::S>(),
                )*]
            }
        }
    };
}

type F32 = f32;
type Color2 = (Color, Color);
type Polynomial4x4 = Polynomial<Vec3, 4, 4>;
make_system!(
    main (GlobalMain) (WorldMain) (Main) (MainPrimPush push):
    - vec3: Vec3;
    - transform: Transform;
    - color: Color;
    - f32: F32;
    - vec2: Vec2;
    - camera: Camera;
    - angle: Angle;
    - dir: Dir;
    - vec4: Vec4;
    - color2: Color2;
    - polynomial4x4: Polynomial4x4;
    store (GlobalStore) (WorldStore) (Stored 0) (StorePrimPush push_stored):
    - transform: Transform;
    - f32: F32;
    - vec2: Vec2;
    - vec3: Vec3;
    - vec4: Vec4;
    - color: Color;
    - color2: Color2;
    - polynomial4x4: Polynomial4x4;
);
impl_write_store!(
    0 => transform Transform (VERTEX),
    1 => f32 F32 (VERTEX),
    2 => vec2 Vec2 (VERTEX),
    3 => vec3 Vec3 (VERTEX),
    4 => vec4 Vec4 (VERTEX),
    5 => color Color (FRAGMENT),
    6 => color2 Color2 (FRAGMENT),
    7 => polynomial4x4 Polynomial4x4 (VERTEX),
);

impl World {
    pub fn allocs(&self, alloc: &mut BufferAllocator) {
        for mat in &self.materials {
            mat.alloc(alloc)
        }
        self.store_allocs(alloc);
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
        self.store_update(&mut ctx.stores);
        self.redraw(&mut ctx);
    }
    pub fn get_cam(&self, idx: isize) -> Camera {
        self.main.camera.get_mod(idx)
    }
}

pub type Main<T> = T;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Stored<T>(pub T);

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
    pub stores: [&'a mut [u32]; GlobalStore::COUNT],
    pub cam: Camera,
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
}

// pub trait Push: Sized + 'static {
//     type Ref;
//     fn push(self, world: &mut World) -> (Self::Ref, GlobalRef);
// }
