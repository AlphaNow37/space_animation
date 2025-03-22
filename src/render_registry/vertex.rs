use crate::render_registry::prefabs::{CIRCLE_POS, FLAT_POS, VertexPoss};
use crate::utils::array_key;
use bytemuck::{Pod, Zeroable};

use super::prefabs::TILED_TRI_POS;

// Bindings
// 0 -> pos_global
// 1 -> local_global_material
// 2 -> global_facts_material
// 3 -> material
// 4 -> tilematrix
// 20 -> pos
// 21 -> tile_pos TODO

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
        Self {
            local_global_material: [local, global, material].map(|i| i as u32),
        }
    }
}

new_vertex!(
    Polynomial4x4Vertex {
        global_facts_material: [u32; 3] : [2 => Uint32x3],
    } -> 3;
);
impl Polynomial4x4Vertex {
    pub fn create(facts: usize, global: usize, material: usize) -> Self {
        Self {
            global_facts_material: [global, facts, material].map(|i| i as u32),
        }
    }
}

new_vertex!(
    TiledTriVertex {
        pos_global: [u32; 4] : [0 => Uint32x4],
        material_tilematrix: [u32; 2] : [3 => Uint32, 4 => Uint32],
    } -> 6;
);
impl TiledTriVertex {
    pub fn create(pos: [usize; 3], tilematrix: usize, global: usize, material: usize) -> Self {
        Self {
            pos_global: [pos[0], pos[1], pos[2], global].map(|i| i as u32),
            material_tilematrix: [material as u32, tilematrix as u32],
        }
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

new_vertex!(
    TilePosVertex {
        pos: [f32; 2]: [21 => Float32x2],
    } -> 2;
);

pub enum AuxiliaryBufferDesc {
    VertexPoss(VertexPoss),
}
array_key!(
    pub enum VertexType {
        Tri,
        Sphere,
        Poly4x4,
        Cube,
        TiledTri,
    }
);
impl VertexType {
    pub fn entry_point(self) -> &'static str {
        match self {
            Self::Tri => "vs_tri",
            Self::Sphere => "vs_sphere",
            Self::Poly4x4 => "vs_poly4x4",
            Self::Cube => "vs_cube",
            Self::TiledTri => "vs_tiled_tri",
        }
    }
    pub fn instance_buffer_label(&self) -> VertexBufferLabel {
        match self {
            Self::Tri => VertexBufferLabel::Tri,
            Self::Sphere => VertexBufferLabel::Sphere,
            Self::Poly4x4 => VertexBufferLabel::Polynomial4x4,
            Self::Cube => VertexBufferLabel::Cube,
            Self::TiledTri => VertexBufferLabel::TiledTri,
        }
    }
    pub fn aux_buffers(&self) -> Vec<AuxiliaryBufferDesc> {
        match self {
            Self::Sphere => vec![AuxiliaryBufferDesc::VertexPoss(*CIRCLE_POS)],
            Self::Poly4x4 => vec![AuxiliaryBufferDesc::VertexPoss(*FLAT_POS)],
            Self::Cube | Self::Tri => Vec::new(),
            Self::TiledTri => vec![AuxiliaryBufferDesc::VertexPoss(*TILED_TRI_POS)],
        }
    }
    pub fn nb_vertex(&self) -> u32 {
        match self {
            Self::Sphere => CIRCLE_POS.len,
            Self::Poly4x4 => FLAT_POS.len,
            Self::Tri => 3,
            Self::Cube => 36,
            Self::TiledTri => TILED_TRI_POS.len,
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
    TilePos,
    Cube,
    TiledTri,
}
impl VertexBufferLabel {
    pub fn elt_size(&self) -> wgpu::BufferAddress {
        match self {
            Self::Tri => TriVertex::SIZE,
            Self::Sphere | Self::Cube => LocalGlobalMatrixVertex::SIZE,
            Self::Polynomial4x4 => Polynomial4x4Vertex::SIZE,
            Self::TiledTri => TiledTriVertex::SIZE,
            Self::Pos3 => Pos3Vertex::SIZE,
            Self::Pos2 => Pos2Vertex::SIZE,
            Self::TilePos => TilePosVertex::SIZE,
        }
    }
    pub fn attrs(&self) -> &'static [wgpu::VertexAttribute] {
        match self {
            Self::Tri => TriVertex::ATTRS,
            Self::Sphere | Self::Cube => LocalGlobalMatrixVertex::ATTRS,
            Self::Polynomial4x4 => Polynomial4x4Vertex::ATTRS,
            Self::TiledTri => TiledTriVertex::ATTRS,
            Self::Pos3 => Pos3Vertex::ATTRS,
            Self::Pos2 => Pos2Vertex::ATTRS,
            Self::TilePos => TilePosVertex::ATTRS,
        }
    }
}
