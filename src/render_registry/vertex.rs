use bytemuck::{Pod, Zeroable};
use crate::math::{Dir, Vec3};
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


/// A compressed normal, either a normalized Dir or Zero
#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct Normal(CompressedVec);
impl From<Dir> for Normal {
    fn from(dir: Dir) -> Self {
        Self::from_normalized(*dir)
    }
}
impl From<Vec3> for Normal {
    fn from(vec: Vec3) -> Self {
        Self::from_vec(vec)
    }
}

impl Normal {
    pub const ZERO: Self = Self([0; 4]);
    pub fn from_normalized(vec: Vec3) -> Self {
        debug_assert!(vec == Vec3::ZERO || vec.is_normalized());
        Self(compress_vec4_i(vec.to_vec4(0.)))
    }
    pub fn from_vec(vec: Vec3) -> Self {
        Self::from_normalized(vec.normalize_or_zero())
    }
    pub fn from_plane(a: Vec3, b: Vec3) -> Self {
        Self::from_normalized(a.cross(b).with_len(1.))
    }
    pub fn from_tri(a: impl Into<Vec3>+Copy, b: impl Into<Vec3>, c: impl Into<Vec3>) -> Self {
        Self::from_plane(a.into()-b.into(), a.into()-c.into())
    }
}

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct TriVertex {
    pub pos: [f32; 3],
    pub normal: Normal,
    pub uv: [f32; 3],
}
impl TriVertex {
    pub fn new(pos: Vec3, normal: Normal, uv: Vec3) -> Self {
        Self {
            pos: pos.to_array(),
            normal,
            uv: uv.to_array(),
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
