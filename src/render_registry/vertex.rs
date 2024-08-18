use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec3A};

use crate::utils::{compress_vec4_i, CompressedVec};

pub trait VertexLike: bytemuck::AnyBitPattern + bytemuck::NoUninit {
    const SIZE: usize;
    const SIZE_U32: usize;
    const ATTRS: &'static [wgpu::VertexAttribute];
    #[allow(dead_code)]
    const _CHECK: ();
}

macro_rules! impl_vertex {
    (
        $ty: ident:
        $size: literal,
        $(
            $n: literal => $val: ident
        ),*
        $(,)?
    ) => {
        impl VertexLike for $ty {
            const ATTRS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
                $(
                    $n => $val,
                )*
            ];
            const SIZE_U32: usize = $size;
            const SIZE: usize = $size * 4;
            const _CHECK: () = const { assert!(std::mem::size_of::<Self>() == Self::SIZE) };

        }
    };
}

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct Normal(CompressedVec);
impl From<Vec3> for Normal {
    fn from(value: Vec3) -> Self {
        Self(compress_vec4_i(value.normalize_or_zero().extend(0.)))
    }
}
impl From<Vec3A> for Normal {
    fn from(value: Vec3A) -> Self {
        Self(compress_vec4_i(value.normalize_or_zero().extend(0.)))
    }
}
impl Normal {
    pub const ZERO: Self = Self([0; 4]);
    pub fn from_normalized(vec: Vec3A) -> Self {
        debug_assert!(vec == Vec3A::ZERO || vec.is_normalized());
        Self(compress_vec4_i(vec.extend(0.)))
    }
    pub fn from_plane(a: impl Into<Vec3A>, b: impl Into<Vec3A>) -> Self {
        a.into().cross(b.into()).into()
    }
    pub fn from_tri(a: impl Into<Vec3A>+Copy, b: impl Into<Vec3A>, c: impl Into<Vec3A>) -> Self {
        Self::from_plane(a.into()-b.into(), a.into()-c.into())
    }
}

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct TriVertex {
    pub pos: Vec3,
    pub normal: Normal,
    pub uv: Vec3,
}
impl TriVertex {
    pub fn new(pos: Vec3A, normal: Normal, uv: Vec3A) -> Self {
        Self {
            pos: pos.into(),
            normal,
            uv: uv.into(),
        }
    }
}
impl_vertex!(TriVertex: 7, 0 => Float32x3, 1 => Snorm8x4, 2 => Float32x3);

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct UniformTriangleVertex {
    pub vertex: TriVertex,
    pub color: CompressedVec,
}
impl_vertex!(UniformTriangleVertex: 8, 0 => Float32x3, 1 => Snorm8x4, 2 => Float32x3, 3 => Unorm8x4, );
impl From<(TriVertex, CompressedVec)> for UniformTriangleVertex {
    fn from((vertex, color): (TriVertex, CompressedVec)) -> Self {
        Self {
            vertex,
            color,
        }
    }
}
impl Into<TriVertex> for UniformTriangleVertex {
    fn into(self) -> TriVertex {
        self.vertex
    }
}
