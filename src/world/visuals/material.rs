use crate::render_registry::{
    alloc::{BufferAllocator},
};
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;

use crate::world::primitives::color::Color;
use crate::world::visuals::VisualDirective;
use crate::world::world::Ref;

pub struct Uniform(pub Ref<Color>);
impl VisualDirective for Uniform {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_mat(self.0.index(), MaterialType::Uniform);
    }
    fn alloc(&self, curr_mty: &mut MaterialType, _alloc: &mut BufferAllocator) {
        *curr_mty = MaterialType::Uniform
    }
}

pub struct Sponge(pub Ref<(Color, Color)>);
impl VisualDirective for Sponge {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_mat(self.0.index(), MaterialType::Sponge);
    }
    fn alloc(&self, curr_mty: &mut MaterialType, _alloc: &mut BufferAllocator) {
        *curr_mty = MaterialType::Sponge
    }
}
