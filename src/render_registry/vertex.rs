use bytemuck::{Pod, Zeroable};
use crate::utils::CompressedVec;

pub trait VertexLike: bytemuck::AnyBitPattern + bytemuck::NoUninit {
    const SIZE: wgpu::BufferAddress;
    const SIZE_U32: wgpu::BufferAddress;
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
            const SIZE_U32: wgpu::BufferAddress = $ssize;
            const SIZE: wgpu::BufferAddress = $ssize * 4;
            const _CHECK: () = const { assert!(std::mem::size_of::<Self>() as wgpu::BufferAddress == Self::SIZE) };

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

// /// A compressed normal, either a normalized Dir or Zero
// #[derive(Pod, Clone, Copy, Zeroable)]
// #[repr(C)]
// pub struct Normal(CompressedVec);
// impl From<Dir> for Normal {
//     fn from(dir: Dir) -> Self {
//         Self::from_normalized(*dir)
//     }
// }
// impl From<Vec3> for Normal {
//     fn from(vec: Vec3) -> Self {
//         Self::from_vec(vec)
//     }
// }
// impl Normal {
//     pub const ZERO: Self = Self([0; 4]);
//     pub fn from_normalized(vec: Vec3) -> Self {
//         debug_assert!(vec == Vec3::ZERO || vec.is_normalized(), "{vec} is not a valid Normal");
//         Self(compress_vec4_i(vec.to_vec4(0.)))
//     }
//     pub fn from_vec(vec: Vec3) -> Self {
//         Self::from_normalized(vec.normalize_or_zero())
//     }
//     pub fn from_plane(a: Vec3, b: Vec3) -> Self {
//         Self::from_normalized(a.cross(b).with_len(1.))
//     }
//     pub fn from_tri(a: impl Into<Vec3>+Copy, b: impl Into<Vec3>, c: impl Into<Vec3>) -> Self {
//         Self::from_plane(a.into()-b.into(), a.into()-c.into())
//     }
// }

new_vertex!(
    TriVertex {
        pos_global: [u32; 4] : [0 => Uint32x4],
        material: u32 : [3 => Uint32]
    } -> 5;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl TriVertex {
    pub fn create(pos: [usize; 3], global: usize, material: usize) -> Self {
        Self {
            pos_global: [pos[0], pos[1], pos[2], global].map(|i| i as u32),
            material: material as u32,
        }
    }
}

new_vertex!(
    LocalGlobalMatrixVertex {
        local_global_material: [u32; 3] : [1 => Uint32x3]
    } -> 3;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl LocalGlobalMatrixVertex {
    pub fn create(local: usize, global: usize, material: usize) -> Self {
        Self {local_global_material: [local, global, material].map(|i| i as u32)}
    }
}

new_vertex!(
    Polynomial4x4Vertex {
        global_facts_material: [u32; 3] : [2 => Uint32x3],
    } -> 3;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);
impl Polynomial4x4Vertex {
    pub fn create(local: usize, facts: usize, material: usize) -> Self {
        Self {global_facts_material: [local, facts, material].map(|i| i as u32)}
    }
}

new_vertex!(
    Pos3Vertex {
        pos: [f32; 3]: [20 => Float32x3],
    } -> 3;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);

new_vertex!(
    Pos2Vertex {
        pos: [f32; 2]: [20 => Float32x2],
    } -> 2;
    new(pos = Self, _shape = ()) -> {pos}
    pos(self) -> {*self}
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexBufferLabel {
    Tri,
    Sphere,
    Polynomial4x4,
    Pos3,
    Pos2,
}
impl VertexBufferLabel {
    pub fn elt_size(&self) -> wgpu::BufferAddress {
        match self {
            Self::Tri => TriVertex::SIZE,
            Self::Sphere => LocalGlobalMatrixVertex::SIZE,
            Self::Polynomial4x4 => Polynomial4x4Vertex::SIZE,
            Self::Pos3 => Pos3Vertex::SIZE,
            Self::Pos2 => Pos2Vertex::SIZE,
        }
    }
    pub fn attrs(&self) -> &'static [wgpu::VertexAttribute] {
        match self {
            Self::Tri => TriVertex::ATTRS,
            Self::Sphere => LocalGlobalMatrixVertex::ATTRS,
            Self::Polynomial4x4 => Polynomial4x4Vertex::ATTRS,
            Self::Pos3 => Pos3Vertex::ATTRS,
            Self::Pos2 => Pos2Vertex::ATTRS,
        }
    }
}
