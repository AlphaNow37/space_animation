
use glam::Vec3A;

use crate::render_registry::{alloc::{BuffersAllocPosition, Position}, pipelines::PipelineLabel, vertex::UniformTriangleVertex};

use super::{color::Color, shape::TriShape, variator::{UpdateCtx, Variator}, world::World};

#[allow(dead_code)]
pub trait TriMeshBuilder {
    /// Push multiple vertex
    fn push_vertexes(&mut self, poss: &[Vec3A]) {
        for p in poss {self.push_vertex(*p);};
    }
    /// Push a single vertex, returns it index
    fn push_vertex(&mut self, pos: Vec3A) -> u32;
    /// Push multiple absolute index
    fn push_indexes(&mut self, idxs: &[u32]) {for i in idxs {self.push_index(*i)}}
    /// Push an absolute index
    fn push_index(&mut self, idx: u32);
    /// The next absolute index
    fn next_index(&self) -> u32;
    /// Push multiple relative index (0=>the next to be pushed)
    fn push_indexes_offset(&mut self, idxs: &mut [u32]) {
        let curr = self.next_index();
        for idx in idxs.iter_mut() {
            *idx += curr;
        }
        self.push_indexes(idxs);
    }
    /// Push a relative index
    fn push_index_offset(&mut self, idx: u32) {self.push_index(idx+self.next_index())}
}
macro_rules! tri_builder {
    (
        $(
            $name: ident {$($attr: ident : $ty: ty),* $(,)?} -> $out: ty [$pos: ident => $vertex: expr]
        );* $(;)?
    ) => {
        $(
            struct $name<'a> {
                next_index: u32,
                vertex: std::slice::ChunksExactMut<'a, u8>,
                index: std::slice::ChunksExactMut<'a, u8>,
                $(
                    $attr: $ty,
                )*
            }
            impl<'a> $name<'a> {
                pub fn new(vertex: &'a mut [u8], index: &'a mut [u8], first_index: u32, $($attr: $ty,)*) -> Self {
                    Self {
                        next_index: first_index,
                        vertex: vertex.chunks_exact_mut(std::mem::size_of::<$out>()),
                        index: index.chunks_exact_mut(4),
                        $($attr,)*
                    }
                }
            }
            impl<'a> TriMeshBuilder for $name<'a> {
                fn push_vertex(&mut self, $pos: Vec3A) -> u32 {
                    let idx = self.next_index;
                    self.next_index += 1;
                    $(
                        let $attr = self.$attr;
                    )*
                    let vertex = &[$vertex];
                    let bytes: &[u8] = bytemuck::cast_slice(vertex);
                    self.vertex.next().unwrap().copy_from_slice(bytes);
                    idx
                }
                fn push_index(&mut self, idx: u32) {
                    self.index.next().unwrap().copy_from_slice(bytemuck::cast_slice(&[idx]));
                }
                fn next_index(&self) -> u32 {
                    self.next_index
                }
            }
        )*
    };
}
tri_builder!(
    UniformTriBuilder {col: u32} -> UniformTriangleVertex [pos => UniformTriangleVertex(pos.into(), col)]
);

pub trait Material {
    fn pipeline(&self) -> PipelineLabel;
    fn nb_index(&self) -> usize;
    fn vertex_bytes(&self) -> usize;
    fn alloc(&self, alloc: &mut BuffersAllocPosition) -> Position {
        alloc.alloc(
            self.pipeline(),
            self.vertex_bytes(),
            self.nb_index()*4,
        )
    }
    fn put(&self, ctx: UpdateCtx, world: &World, vertex: &mut [u8], index_offset: u32, index: &mut [u8]);
}

pub struct UniformTri<S, C> {
    pub shape: S,
    pub color: C,
}
impl<S: TriShape, C: Variator<Item=Color>> Material for UniformTri<S, C> {
    fn pipeline(&self) -> PipelineLabel {
        PipelineLabel::UniformTriangle
    }
    fn nb_index(&self) -> usize {
        self.shape.nb_index()
    }
    fn vertex_bytes(&self) -> usize {
        self.shape.nb_vertex() * UniformTriangleVertex::SIZE
    }
    fn put(&self, ctx: UpdateCtx, world: &World, vertex: &mut [u8], index_offset: u32, index: &mut [u8]) {
        let col = self.color.update(ctx, world).as_u32();
        let mut builder = UniformTriBuilder::new(vertex, index, index_offset/UniformTriangleVertex::SIZE as u32, col);
        self.shape.put(&mut builder, ctx, world);
    }
}
