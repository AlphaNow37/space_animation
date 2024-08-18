use std::ops::Range;

use crate::render_registry::{
    alloc::{BuffersAllocPosition, Position},
    pipelines::PipelineLabel,
    vertex::UniformTriangleVertex,
};

use crate::world::{
    primitives::color::Color,
    visuals::mesh_builder::BaseTriMeshBuilder,
    variators::variator::{UpdateCtx, Variator},
    world::World,
};
use crate::world::visuals::shape::TriShape;

pub trait Material {
    fn pipeline(&self) -> PipelineLabel;
    fn nb_index(&self) -> usize;
    fn nb_vertex(&self) -> usize;
    fn alloc(&self, alloc: &mut BuffersAllocPosition) -> Position {
        alloc.alloc(self.pipeline(), self.nb_vertex(), self.nb_index())
    }
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        vertex: &mut [u32],
        vertex_offset_bounds: Range<usize>,
        index: &mut [u32],
    );
}

pub struct UniformTri<S, C> {
    pub shape: S,
    pub color: C,
}
impl<S: TriShape, C: Variator<Item = Color>> Material for UniformTri<S, C> {
    fn pipeline(&self) -> PipelineLabel {
        PipelineLabel::UniformTriangle
    }
    fn nb_index(&self) -> usize {
        // self.shape.nb_index()
        S::NB_INDEX
    }
    fn nb_vertex(&self) -> usize {
        S::NB_VERTEX
    }
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        vertex: &mut [u32],
        vertex_offset_bounds: Range<usize>,
        index: &mut [u32],
    ) {
        let mut builder: BaseTriMeshBuilder<[u8; 4], UniformTriangleVertex> =
            BaseTriMeshBuilder::new(
                vertex,
                index,
                self.color.update(ctx, world).as_array(),
                vertex_offset_bounds,
            );
        self.shape.put(&mut builder, ctx, world);
    }
}
