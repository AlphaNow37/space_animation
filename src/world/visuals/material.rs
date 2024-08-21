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
        let global = self.global.update(ctx, world);
        let color = self.color.update(ctx, world).as_array();
        self.shape.put(builders.uniform_triangle.with_global_data(global, color), ctx, world);
    }
}
pub struct SpongeTri<S, C1, C2, G> {
    pub global: G,
    pub shape: S,
    pub color1: C1,
    pub color2: C2,
}
impl<S: TriShape, C1: Variator<Item = Color>, C2: Variator<Item = Color>, G: Variator<Item=Transform>> Material for SpongeTri<S, C1, C2, G> {
    fn alloc(&self, alloc: &mut BuffersAllocPosition) {
        alloc.alloc(PipelineLabel::SpongeTriangle, S::NB_VERTEX, S::NB_INDEX);
    }
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        builders: &mut MeshBuilders,
    ) {
        let global = self.global.update(ctx, world);
        let color1 = self.color1.update(ctx, world).as_array();
        let color2 = self.color2.update(ctx, world).as_array();
        self.shape.put(builders.sponge_triangle.with_global_data(global, (color1, color2)), ctx, world);
    }
}
