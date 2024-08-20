use crate::math::Transform;

use crate::render_registry::{
    alloc::{BuffersAllocPosition},
    pipelines::PipelineLabel,
};
use crate::render_registry::mesh_builder::MeshBuilders;

use crate::world::{
    primitives::color::Color,
    variators::variator::{UpdateCtx, Variator},
    world::World,
};
use crate::world::visuals::shape::TriShape;

pub trait Material {
    fn alloc(&self, alloc: &mut BuffersAllocPosition);
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        builders: &mut MeshBuilders,
    );
}

pub struct UniformTri<S, C, G> {
    pub global: G,
    pub shape: S,
    pub color: C,
}
impl<S: TriShape, C: Variator<Item = Color>, G: Variator<Item=Transform>> Material for UniformTri<S, C, G> {
    fn alloc(&self, alloc: &mut BuffersAllocPosition) {
        alloc.alloc(PipelineLabel::UniformTriangle, S::NB_VERTEX, S::NB_INDEX);
    }
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        builders: &mut MeshBuilders,
    ) {
        let builder = &mut builders.uniform_triangle;
        builder.data = self.color.update(ctx, world).as_array();
        builder.global = self.global.update(ctx, world);
        self.shape.put(builder, ctx, world);
    }
}
