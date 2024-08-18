use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec3A};

use crate::utils::{compress_vec4_i, compress_vec4_u, CompressedVec};

pub trait VertexLike: bytemuck::NoUninit {
    const SIZE: usize;
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
            const SIZE: usize = $size * 4;
            const _CHECK: () = const { assert!(std::mem::size_of::<Self>() == Self::SIZE) };
        }
    };
}

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct UniformTriangleVertex {
    pub pos: Vec3,
    pub color: CompressedVec,
    pub uv: Vec3,
    pub normal: CompressedVec,
}
impl_vertex!(UniformTriangleVertex: 8, 0 => Float32x3, 1 => Unorm8x4, 2 => Float32x3, 3 => Snorm8x4);
impl UniformTriangleVertex {
    pub fn new(pos: Vec3A, color: CompressedVec, uv: Vec3A, normal: Vec3A) -> Self {
        debug_assert!(normal==Vec3A::ZERO || normal.is_normalized());
        Self {
            pos: pos.into(),
            color,
            uv: uv.into(),
            normal: compress_vec4_i(normal.extend(0.)),
        }
    }
}
