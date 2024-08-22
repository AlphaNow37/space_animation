use bytemuck::{Pod, Zeroable};
use crate::math::{Dir, Transform, Vec3};
use crate::utils::{compress_vec4_i, CompressedVec, Zero};

pub trait VertexLike: bytemuck::AnyBitPattern + bytemuck::NoUninit {
    const SIZE: usize;
    const SIZE_U32: usize;
    const ATTRS: &'static [wgpu::VertexAttribute];
    #[allow(dead_code)]
    const _CHECK: ();

    type PosData;
    type ShapeData: Copy + Default;
    fn new(pos: Self::PosData, shape: Self::ShapeData) -> Self;
    fn pos(&self) -> Self::PosData;
}

macro_rules! new_vertex {
    (
        $sname: ident {
            $(
                $attr: ident : $ty: ty : [$($idx: literal => $pack: ident),* $(,)?]
            ),* $(,)?
        } -> $ssize: expr;

        new($pos: pat = $posty: ty , $shape: pat = $shapety: ty)
        -> {$new: expr}

        pos($self: ident) -> {$getpos: expr}
    ) => {
        #[derive(Pod, Clone, Copy, Zeroable)]
        #[repr(C)]
        pub struct $sname {
            $(pub $attr: $ty),*
        }
        impl VertexLike for $sname {
            const ATTRS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
                 $(
                    $(
                        $idx => $pack,
                    )*
                )*
            ];
            const SIZE_U32: usize = $ssize;
            const SIZE: usize = $ssize * 4;
            const _CHECK: () = const { assert!(std::mem::size_of::<Self>() == Self::SIZE) };

            type PosData = $posty;
            type ShapeData = $shapety;
            fn new($pos: Self::PosData, $shape: Self::ShapeData) -> Self {
                $new
            }
            fn pos(&$self) -> Self::PosData {
                $getpos
            }
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
        debug_assert!(vec == Vec3::ZERO || vec.is_normalized(), "{vec} is not a valid Normal");
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

new_vertex!(
    TriVertex {
        pos: [f32; 3] : [0 => Float32x3],
        normal: Normal : [1 => Snorm8x4],
        uv: [f32; 3] : [2 => Float32x3],
    } -> 7;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl TriVertex {
    pub fn create(pos: Vec3, normal: impl Into<Normal>, uv: Vec3) -> Self {
        Self {
            pos: pos.to_array(),
            normal: normal.into(),
            uv: uv.to_array(),
        }
    }
}

new_vertex!(
    SphereVertex {
        local: [f32; 16] : [0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4],
        global: [f32; 16] : [4 => Float32x4, 5 => Float32x4, 6 => Float32x4, 7 => Float32x4],
    } -> 32;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl SphereVertex {
    pub fn create(global: Transform, local: Transform) -> Self {
        Self {
            local: local.to_mat4().to_array(),
            global: global.to_mat4().to_array(),
        }
    }
}

new_vertex!(
    PosVertex {
        pos: [f32; 3]: [20 => Float32x3],
    } -> 3;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl PosVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {pos: [x, y, z]}
    }
}

new_vertex!(
    TriVertexCol1 {
        vertex: TriVertex : [0 => Float32x3, 1 => Snorm8x4, 2 => Float32x3],
        color: CompressedVec : [10 => Snorm8x4],
    } -> 8;
    new(vertex = TriVertex, color = CompressedVec) -> {Self {vertex, color}}
    pos(self) -> {self.vertex}
);
new_vertex!(
    TriVertexCol2 {
        vertex: TriVertex : [0 => Float32x3, 1 => Snorm8x4, 2 => Float32x3],
        col1: CompressedVec : [10 => Snorm8x4],
        col2: CompressedVec : [11 => Snorm8x4],
    } -> 9;
    new(vertex = TriVertex, (col1, col2) = (CompressedVec, CompressedVec)) -> {Self {vertex, col1, col2}}
    pos(self) -> {self.vertex}
);
new_vertex!(
    SphereVertexCol1 {
        vertex: SphereVertex : [0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x4, 7 => Float32x4],
        color: CompressedVec : [10 => Snorm8x4],
    } -> 33;
    new(vertex = SphereVertex, color = CompressedVec) -> {Self {vertex, color}}
    pos(self) -> {self.vertex}
);
