use camera::Camera;
use color::Color;

use crate::math::{Angle, Dir, Polynomial, Transform, Vec2, Vec3, Vec4};
use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::storage_structs::AsStrorageStruct;
use crate::utils::array_key;

use std::cell::Cell;

pub mod camera;
pub mod color;

pub trait WorldPrimitive: Sized + 'static {
    fn alloc(world: &mut PrimitivesAllocationTracker, size: usize) -> usize;
    fn get(stores: &PrimitiveStoresHolder, index: usize) -> Self;
    fn set(stores: &PrimitiveStoresHolder, index: usize, value: Self);
    fn sets<const N: usize>(stores: &PrimitiveStoresHolder, index: usize, values: [Self; N]);
}

macro_rules! make_primitive_system {
    (
        $(
            $snake_name: ident : $prim_ty: ty $({$store_name: ident})?
        );*
        $(;)?
    ) => {
        pub struct PrimitiveStoresHolder {
            $(
                $snake_name: Vec<Cell<$prim_ty>>,
            )*
        }
        impl Default for PrimitiveStoresHolder {
            fn default() -> Self {
                Self {
                    $(
                        $snake_name: Vec::new(),
                    )*
                }
            }
        }

        pub struct PrimitivesAllocationTracker {
            $(
                $snake_name: usize,
            )*
        }
        impl Default for PrimitivesAllocationTracker {
            fn default() -> Self {
                Self {
                    $(
                        $snake_name: 0,
                    )*
                }
            }
        }
        impl PrimitivesAllocationTracker {
            pub fn allocs(&self, alloc: &mut BufferAllocator) {
                $(
                    $(
                        alloc.alloc_store(StoreLabel::$store_name, self.$snake_name);
                    )?
                )*
            }
            pub fn to_store_holder(&self) -> PrimitiveStoresHolder {
                PrimitiveStoresHolder {
                    $(
                        $snake_name: vec![Cell::new(<$prim_ty>::default()); self.$snake_name],
                    )*
                }
            }
        }

        $(
            impl WorldPrimitive for $prim_ty {
                fn alloc(tracker: &mut PrimitivesAllocationTracker, size: usize) -> usize {
                    let idx = tracker.$snake_name;
                    tracker.$snake_name += size;
                    idx
                }
                fn get(stores: &PrimitiveStoresHolder, index: usize) -> Self {
                    stores.$snake_name[index].get()
                }
                fn set(stores: &PrimitiveStoresHolder, index: usize, value: Self) {
                    stores.$snake_name[index].set(value);
                }
                fn sets<const N: usize>(stores: &PrimitiveStoresHolder, index: usize, values: [Self; N]) {
                    for i in 0..values.len() {
                        Self::set(stores, index+i, values[i]) // i hope this gets optimised away
                    }
                }
            }
        )*

        array_key!(
            pub enum StoreLabel {
                $(
                    $($store_name, )?
                )*
            }
        );
        impl StoreLabel {
            pub fn write(self, buf: &mut [u32], stores: &PrimitiveStoresHolder) {
                match self {
                    $(
                        $(
                            Self::$store_name => {
                                let arr: &mut [<$prim_ty as AsStrorageStruct>::S] = bytemuck::cast_slice_mut(buf);
                                for i in 0..stores.$snake_name.len() {
                                    arr[i] = stores.$snake_name[i].get().as_strorage_struct();
                                }
                            }
                        )?
                    )*
                }
            }
        }
    };
}

type F32 = f32;
type Color2 = (Color, Color);
type Polynomial4x4 = Polynomial<Vec3, 4, 4>;
make_primitive_system!(
    vec3: Vec3 {Vec3};
    transform: Transform {Transform};
    color: Color {Color};
    f32: F32 {F32};
    vec2: Vec2 {Vec2};
    camera: Camera;
    angle: Angle;
    dir: Dir;
    vec4: Vec4 {Vec4};
    color2: Color2 {Color2};
    polynomial4x4: Polynomial4x4 {Poly4x4};
);

impl PrimitiveStoresHolder {
    pub fn nb_cameras(&self) -> usize {
        self.camera.len()
    }
}

impl StoreLabel {
    pub fn struct_size(self) -> usize {
        16 * match self {
            Self::F32 | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Color => 1,
            Self::Color2 => 2,
            Self::Poly4x4 => 16,
            Self::Transform => 4,
        }
    }
    pub fn bind(self) -> u32 {
        match self {
            Self::Transform => 0,
            Self::F32 => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            Self::Color => 5,
            Self::Color2 => 6,
            Self::Poly4x4 => 7,
        }
    }
    pub fn stage(self) -> wgpu::ShaderStages {
        match self {
            Self::Transform | Self::F32 | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Poly4x4 => {
                wgpu::ShaderStages::VERTEX
            }
            Self::Color | Self::Color2 => wgpu::ShaderStages::FRAGMENT,
        }
    }
}
