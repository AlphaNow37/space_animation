
use glam::Vec3A;

use crate::render_registry::{alloc::{BuffersAllocPosition, Position}, pipelines::PipelineLabel, vertex::{UniformTriangleVertex, VertexLike}};

use super::{color::Color, shape::TriShape, variator::{UpdateCtx, Variator}, world::World};

/// (Pos, Normal, Uv)
pub type Vertex = (Vec3A, Vec3A, Vec3A);
#[allow(dead_code)]
pub trait TriMeshBuilder {
    /// Push multiple vertex (Pos, Normal, Uv)
    fn push_vertexes(&mut self, vertexes: &[Vertex]) {
        for v in vertexes {self.push_vertex(*v);};
    }
    /// Push a single vertex, returns it index (Pos, Normal, Uv)
    fn push_vertex(&mut self, vertex: Vertex) -> u32;
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

struct BaseMeshTriBuilder<'a, D, F> {
    next_index: u32,
    vertex: std::slice::ChunksExactMut<'a, u8>,
    index: &'a mut [u8],
    data: D,
    f: F,
}
impl<'a, D, T: VertexLike, F: Fn(Vertex, &D)->T> BaseMeshTriBuilder<'a, D, F> {
    pub fn new(vertex: &'a mut [u8], index: &'a mut [u8], first_index: u32, data: D, f: F) -> Self {
        Self {
            next_index: first_index/T::SIZE as u32,
            vertex: vertex.chunks_exact_mut(T::SIZE),
            index, //: index.chunks_exact_mut(4),
            data,
            f,
        }
    }
}
impl<'a, D, T: VertexLike, F: Fn(Vertex, &D)->T> TriMeshBuilder for BaseMeshTriBuilder<'a, D, F> {
    fn push_vertex(&mut self, vertex: Vertex) -> u32 {
        let idx = self.next_index;
        self.next_index += 1;
        let vertex = &[(self.f)(vertex, &self.data)];
        let bytes: &[u8] = bytemuck::cast_slice(vertex);
        self.vertex.next().unwrap().copy_from_slice(bytes);
        idx
    }
    fn push_index(&mut self, idx: u32) {
        let a;
        (a, self.index) = std::mem::take(&mut self.index).split_at_mut(4);
        a.copy_from_slice(&idx.to_be_bytes());
    }
    fn push_indexes(&mut self, idxs: &[u32]) {
        let a;
        (a, self.index) = std::mem::take(&mut self.index).split_at_mut(idxs.len()*4);
        a.copy_from_slice(bytemuck::cast_slice(idxs));
    }
    fn next_index(&self) -> u32 {
        self.next_index
    }
}

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
        let mut builder = BaseMeshTriBuilder::new(
            vertex, 
            index, 
            index_offset,
            self.color.update(ctx, world).as_array(),
            |(pos, normal, uv), color| UniformTriangleVertex::new(pos, *color, uv, normal)
        );
        self.shape.put(&mut builder, ctx, world);
    }
}
