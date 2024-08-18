use std::ops::Range;

use wgpu::hal::auxil::db;

use crate::render_registry::vertex::{TriVertex, VertexLike};

#[allow(dead_code)]
pub trait TriMeshBuilder {
    /// Push multiple vertex (Pos, Normal, Uv)
    fn push_vertexes<const N: usize>(&mut self, vertexes: [TriVertex; N]);
    /// Push a single vertex, returns it index (Pos, Normal, Uv)
    fn push_vertex(&mut self, vertex: TriVertex);
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
    fn get_vertex(&self, idx: u32) -> TriVertex;
}

// #[allow(dead_code)]
// pub trait TriMeshBuilder {
//     /// Push a single vertex, returns it absolute index (Pos, Normal, Uv)
//     fn push_vertex(&mut self, vertex: Vertex);
//     fn transform_index(&self, idx: u32) -> u32;
//     /// Push multiple absolute index
//     fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]);

//     /// Push multiple vertex (Pos, Normal, Uv)
//     fn push_vertexes(&mut self, vertexes: &[Vertex]) {
//         for v in vertexes {self.push_vertex(*v);};
//     }

//     // /// The next absolute index
//     // fn next_index(&self) -> u32;
//     /// Push multiple relative index (0=>the next to be pushed)
//     fn push_indexes_transform<const N: usize>(&mut self, idxs: [u32; N]) {
//         self.push_indexes(idxs.map(|i| self.transform_index(i)));
//     }

//     fn offset(&mut self, offset: u32) -> OffsetTriMeshBuilder<Self> where Self: Sized {
//         OffsetTriMeshBuilder(self, offset)
//     }
//     fn step(&mut self, step: u32) -> StepTriMeshBuilder<Self> where Self: Sized {
//         StepTriMeshBuilder(self, step)
//     }
//     fn cache_pos<'a, 'b>(&'a mut self, cache: &'a mut [Vec3A]) -> CachedTriMeshBuilder<'a, Self> where Self: Sized {
//         CachedTriMeshBuilder {
//             cache,
//             curr_cache_index: 0,
//             builder: self,
//         }
//     }
// }

// pub struct OffsetTriMeshBuilder<'a, M>(pub &'a mut M, pub u32);
// impl<'a, M: TriMeshBuilder> TriMeshBuilder for OffsetTriMeshBuilder<'a, M> {
//     fn push_vertex(&mut self, vertex: Vertex) {
//         self.0.push_vertex(vertex)
//     }
//     fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]) {
//         self.0.push_indexes(idxs)
//     }
//     fn transform_index(&self, idx: u32) -> u32 {
//         idx + self.1
//     }
// }

// pub struct StepTriMeshBuilder<'a, M>(pub &'a mut M, pub u32);
// impl<'a, M: TriMeshBuilder> TriMeshBuilder for StepTriMeshBuilder<'a, M> {
//     fn push_vertex(&mut self, vertex: Vertex) {
//         self.0.push_vertex(vertex)
//     }
//     fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]) {
//         self.0.push_indexes(idxs)
//     }
//     fn transform_index(&self, idx: u32) -> u32 {
//         idx * self.1
//     }
// }


// pub struct CachedTriMeshBuilder<'a, M> {
//     cache: &'a mut [Vec3A],
//     curr_cache_index: usize,
//     pub builder: &'a mut M,
// }
// impl<'a, M: TriMeshBuilder> TriMeshBuilder for CachedTriMeshBuilder<'a, M> {
//     fn push_vertex(&mut self, vertex: Vertex) {
//         self.cache[self.curr_cache_index] = vertex.0;
//         self.curr_cache_index += 1;
//         self.builder.push_vertex(vertex);
//     }
//     fn push_indexes<const N: usize>(&mut self, idxs: [u32; N]) {
//         self.builder.push_indexes(idxs);
//     }
//     fn transform_index(&self, idx: u32) -> u32 {
//         self.builder.transform_index(idx)
//     }
// }


pub struct BaseTriMeshBuilder<'a, D, T> {
    bound: Range<usize>,
    vertex: &'a mut [T],
    index: &'a mut [u32],
    data: D,
}
impl<'a, D, T: VertexLike> BaseTriMeshBuilder<'a, D, T> {
    pub fn new(vertex: &'a mut [u32], index: &'a mut [u32], data: D, vertex_offset_bounds: Range<usize>,) -> Self {
        Self {
            bound: vertex_offset_bounds.clone(),
            vertex: bytemuck::cast_slice_mut(vertex),
            index, //: index.chunks_exact_mut(4),
            data,
        }
    }
}
impl<'a, D: Copy, T: VertexLike+for<'b> From<(TriVertex, D)>+Into<TriVertex>> TriMeshBuilder for BaseTriMeshBuilder<'a, D, T> {
    fn push_vertex(&mut self, vertex: TriVertex) {
        self.vertex[self.bound.next().unwrap()] = T::from((vertex, self.data));
    }
    fn push_vertexes<const N: usize>(&mut self, vertexes: [TriVertex; N]) {
        if N==0 {return;}
        let start = self.bound.start;
        let end = self.bound.start + N;
        if end > self.bound.end {panic!("Took too much place !")}
        self.bound.start = end;
        (&mut self.vertex[start..end])
            .copy_from_slice(&vertexes.map(|v| T::from((v, self.data))));
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
    fn get_vertex(&self, idx: u32) -> TriVertex {
        self.vertex[idx as usize].into()
    }
}
