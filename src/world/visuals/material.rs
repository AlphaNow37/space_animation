use crate::math::Transform;

use crate::render_registry::{
    alloc::{BufferAllocator},
    pipelines::PipelineLabel,
};
use crate::render_registry::mesh_builder::{MeshBuilder, MeshBuilders};
use crate::render_registry::vertex::{LocalGlobalMatrixVertex};

use crate::world::{
    primitives::color::Color,
    variators::variator::{UpdateCtx, Variator},
    world::World,
};
use crate::world::visuals::shape::TriShape;
use crate::world::world::{Ref, Stored};

pub trait Material {
    fn alloc(&self, alloc: &mut BufferAllocator);
    fn put(
        &self,
        ctx: UpdateCtx,
        world: &World,
        builders: &mut MeshBuilders,
    );
}

pub struct UniformTri<S> {
    pub global: Ref<Stored<Transform>>,
    pub shape: S,
    pub color: Ref<Stored<Color>>,
}
impl<S: TriShape> Material for UniformTri<S> {
    fn alloc(&self, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(PipelineLabel::UniformTriangle, S::NB_INSTANCE);
    }
    fn put(
        &self,
        _ctx: UpdateCtx,
        _world: &World,
        builders: &mut MeshBuilders,
    ) {
        // let global = self.global.update(ctx, world);
        // let color = self.color.update(ctx, world).as_array();
        self.shape.put(&mut builders.uniform_triangle, self.global.index(), self.color.index());
    }
}
// pub struct SpongeTri<S, C1, C2, G> {
//     pub global: Ref<Stored<Transform>>,
//     pub shape: S,
//     pub colors: Ref<Stored<(Color, Color)>>,
// }
// impl<S: TriShape> Material for SpongeTri<S> {
//     fn alloc(&self, alloc: &mut BufferAllocator) {
//         alloc.alloc_vertex(PipelineLabel::SpongeTriangle, S::NB_VERTEX);
//     }
//     fn put(
//         &self,
//         _ctx: UpdateCtx,
//         _world: &World,
//         builders: &mut MeshBuilders,
//     ) {
//         // let global = self.global.update(ctx, world);
//         // let color1 = self.color1.update(ctx, world).as_array();
//         // let color2 = self.color2.update(ctx, world).as_array();
//         self.shape.put(builders.sponge_triangle.with_global(global).with_data((color1, color2)), ctx, world);
//     }
// }
pub struct UniformSphere {
    pub global: Ref<Stored<Transform>>,
    pub local: Ref<Stored<Transform>>,
    pub color: Ref<Stored<Color>>,
}
impl Material for UniformSphere {
    fn alloc(&self, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(PipelineLabel::UniformSphere, 1);
    }
    fn put(
        &self,
        _ctx: UpdateCtx,
        _world: &World,
        builders: &mut MeshBuilders,
    ) {
        // let global = self.global.update(ctx, world);
        // let local = self.local.update(ctx, world);
        // let color = self.color.update(ctx, world).as_array();
        // builders.uniform_sphere.with_data(color).push_vertex(SphereVertex::create(global, local));
        builders.uniform_sphere.push_vertex(LocalGlobalMatrixVertex::create(
            self.local.index(),
            self.global.index(),
            self.color.index(),
        ))
    }
}
