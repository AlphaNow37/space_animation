use bytemuck::NoUninit;
use crate::render_registry::materials::{MaterialRef, MaterialType};

use crate::render_registry::vertex::{LocalGlobalMatrixVertex, Polynomial4x4Vertex, TriVertex, VertexType};

use super::vertex::TiledTriVertex;

pub struct VisualExecutor<'a> {
    curr_global: usize,
    curr_mat: MaterialRef,
    bufs: [[&'a mut [u32]; MaterialType::COUNT]; VertexType::COUNT],
}
impl<'a> VisualExecutor<'a> {
    pub fn new(bufs: [[&'a mut [u32]; MaterialType::COUNT]; VertexType::COUNT]) -> Self {
        Self {
            curr_global: 0,
            curr_mat: MaterialRef::default(),
            bufs,
        }
    }
    fn push(&mut self, vty: VertexType, data: impl NoUninit) {
        let buf = &mut self.bufs[vty as usize][self.curr_mat.mty as usize];
        let array = [data];
        let slice: &[u32] = bytemuck::cast_slice(&array);
        let a;
        (a, *buf) = std::mem::take(buf).split_at_mut(slice.len());
        a.copy_from_slice(slice);
    }
    pub fn set_mat(&mut self, mat: MaterialRef) {
        self.curr_mat = mat;
    }
    pub fn set_global(&mut self, global: usize) {
        self.curr_global = global;
    }
    pub fn push_tri(&mut self, pts: [usize; 3]) {
        self.push(VertexType::Tri, TriVertex::create(pts, self.curr_global, self.curr_mat.index))
    }
    pub fn push_sphere(&mut self, tr: usize) {
        self.push(VertexType::Sphere, LocalGlobalMatrixVertex::create(tr, self.curr_global, self.curr_mat.index))
    }
    pub fn push_cube(&mut self, tr: usize) {
        self.push(VertexType::Cube, LocalGlobalMatrixVertex::create(tr, self.curr_global, self.curr_mat.index))
    }
    pub fn push_poly4x4(&mut self, facts: usize) {
        self.push(VertexType::Poly4x4, Polynomial4x4Vertex::create(facts, self.curr_global, self.curr_mat.index))
    }
    pub fn push_tiled_tri(&mut self, pts: [usize; 3], tilematrix: usize) {
        self.push(VertexType::TiledTri, TiledTriVertex::create(pts, tilematrix, self.curr_global, self.curr_mat.index))
    }
}
