use bytemuck::{Pod, Zeroable};
use crate::render_registry::prefabs::{CIRCLE_POS, FLAT_POS, VertexPoss};
use crate::utils::array_key;

pub trait VertexLike: bytemuck::AnyBitPattern + bytemuck::NoUninit {
    const SIZE: wgpu::BufferAddress;
    const SIZE_U32: wgpu::BufferAddress;
    const ATTRS: &'static [wgpu::VertexAttribute];
    #[allow(dead_code)]
    const _CHECK: ();
}

macro_rules! new_vertex {
    (
        $sname: ident {
            $(
                $attr: ident : $ty: ty : [$($idx: literal => $pack: ident),* $(,)?]
            ),* $(,)?
        } -> $ssize: expr;
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
        }
    };
}

new_vertex!(
    TriVertex {
        pos_global: [u32; 4] : [0 => Uint32x4],
        material: u32 : [3 => Uint32]
    } -> 5;
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
);

new_vertex!(
    Pos2Vertex {
        pos: [f32; 2]: [20 => Float32x2],
    } -> 2;
);

pub enum SecondaryBufferDesc {
    VertexPoss(VertexPoss),
    None,
}
array_key!(
    pub enum VertexType {
        Tri,
        Sphere,
        Poly4x4,
        Cube,
    }
);
impl VertexType {
    pub fn entry_point(self) -> &'static str {
        match self {
            Self::Tri => "vs_tri",
            Self::Sphere => "vs_sphere",
            Self::Poly4x4 => "vs_poly4x4",
            Self::Cube => "vs_cube",
        }
    }
    pub fn instance_buffer_label(&self) -> VertexBufferLabel {
        match self {
            Self::Tri => VertexBufferLabel::Tri,
            Self::Sphere => VertexBufferLabel::Sphere,
            Self::Poly4x4 => VertexBufferLabel::Polynomial4x4,
            Self::Cube => VertexBufferLabel::Cube,
        }
    }
    pub fn secondary_buffer(&self) -> SecondaryBufferDesc {
        match self {
            Self::Sphere => SecondaryBufferDesc::VertexPoss(*CIRCLE_POS),
            Self::Poly4x4 => SecondaryBufferDesc::VertexPoss(*FLAT_POS),
            Self::Cube => SecondaryBufferDesc::None,
            Self::Tri => SecondaryBufferDesc::None,
        }
    }
    pub fn nb_vertex(&self) -> u32 {
        match self {
            Self::Sphere => CIRCLE_POS.len,
            Self::Poly4x4 => FLAT_POS.len,
            Self::Tri => 3,
            Self::Cube => 36,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexBufferLabel {
    Tri,
    Sphere,
    Polynomial4x4,
    Pos3,
    Pos2,
    Cube,
}
impl VertexBufferLabel {
    pub fn elt_size(&self) -> wgpu::BufferAddress {
        match self {
            Self::Tri => TriVertex::SIZE,
            Self::Sphere | Self::Cube => LocalGlobalMatrixVertex::SIZE,
            Self::Polynomial4x4 => Polynomial4x4Vertex::SIZE,
            Self::Pos3 => Pos3Vertex::SIZE,
            Self::Pos2 => Pos2Vertex::SIZE,
        }
    }
    pub fn attrs(&self) -> &'static [wgpu::VertexAttribute] {
        match self {
            Self::Tri => TriVertex::ATTRS,
            Self::Sphere | Self::Cube => LocalGlobalMatrixVertex::ATTRS,
            Self::Polynomial4x4 => Polynomial4x4Vertex::ATTRS,
            Self::Pos3 => Pos3Vertex::ATTRS,
            Self::Pos2 => Pos2Vertex::ATTRS,
        }
    }
}
