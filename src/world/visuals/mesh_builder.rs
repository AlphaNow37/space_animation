use std::ops::Range;
use crate::math::Transform;

use crate::render_registry::vertex::{TriVertex, VertexLike};
use crate::utils::make_trait_alias;

#[allow(dead_code)]
pub trait MeshBuilder {
    type Vertex;

    /// Push multiple vertex (Pos, Normal, Uv)
    fn push_vertexes<const N: usize>(&mut self, vertexes: [Self::Vertex; N]);
    /// Push a single vertex, returns it index (Pos, Normal, Uv)
    fn push_vertex(&mut self, vertex: Self::Vertex);
    /// Push multiple absolute index
    fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]);
    /// Push an absolute index
    fn push_index(&mut self, idx: u32);
    /// The next absolute index
    fn next_index(&self) -> u32;
    /// Push multiple relative index (0=>the next to be pushed)
    fn push_indexes_offset<const N: usize>(&mut self, idxs: [u32; N]) {
        let curr = self.next_index();
        self.push_indexes(idxs.map(|i| i+curr));
    }
    /// Push a relative index
    fn push_index_offset(&mut self, idx: u32) {self.push_index(idx+self.next_index())}
    /// retrieve info about another vertex
    fn get_vertex(&self, idx: u32) -> Self::Vertex;
    /// The global transform, don't affect uvs
    fn global(&self) -> &Transform;
}

make_trait_alias!(TriMeshBuilder = MeshBuilder<Vertex=TriVertex>);

pub struct BaseTriMeshBuilder<'a, T: VertexLike> {
    bound: Range<usize>,
    vertex: &'a mut [T],
    index: &'a mut [u32],
    data: T::ShapeData,
    global: Transform,
}
impl<'a, T: VertexLike> BaseTriMeshBuilder<'a, T> {
    pub fn new(vertex: &'a mut [u32], index: &'a mut [u32], data: T::ShapeData, vertex_offset_bounds: Range<usize>, global: Transform) -> Self {
        Self {
            bound: vertex_offset_bounds.clone(),
            vertex: bytemuck::cast_slice_mut(vertex),
            index, //: index.chunks_exact_mut(4),
            data,
            global,
        }
    }
}
impl<'a, T: VertexLike> MeshBuilder for BaseTriMeshBuilder<'a, T> {
    type Vertex = T::PosData;
    fn push_vertexes<const N: usize>(&mut self, vertexes: [Self::Vertex; N]) {
        if N==0 {return;}
        let start = self.bound.start;
        let end = self.bound.start + N;
        if end > self.bound.end {panic!("Took too much place !")}
        self.bound.start = end;
        (&mut self.vertex[start..end])
            .copy_from_slice(&vertexes.map(|v| T::new(v, self.data)));
    }
    fn push_vertex(&mut self, vertex: Self::Vertex) {
        self.vertex[self.bound.next().unwrap()] = T::new(vertex, self.data);
    }
    fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]) {
        let a;
        (a, self.index) = std::mem::take(&mut self.index).split_at_mut(N);
        a.copy_from_slice(&idxs);
    }
    fn push_index(&mut self, idx: u32) {
        self.push_indexes([idx])
    }
    fn next_index(&self) -> u32 {
        self.bound.start as u32
    }
    fn get_vertex(&self, idx: u32) -> Self::Vertex {
        self.vertex[idx as usize].pos()
    }
    fn global(&self) -> &Transform {
        &self.global
    }
}
