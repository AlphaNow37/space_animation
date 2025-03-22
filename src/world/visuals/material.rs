use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::materials::{MaterialRef, MaterialType};
use crate::render_registry::mesh_builder::VisualExecutor;

use crate::world::primitives::color::Color;
use crate::world::visuals::VisualDirective;
use crate::world::world::Ref;

impl VisualDirective for Ref<Color> {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_mat(MaterialRef {
            index: self.index(),
            mty: MaterialType::Uniform,
        })
    }
    fn alloc(&self, curr_mty: &mut MaterialType, _alloc: &mut BufferAllocator) {
        *curr_mty = MaterialType::Uniform;
    }
}

pub struct Sponge(pub Ref<(Color, Color)>);
impl VisualDirective for Sponge {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_mat(MaterialRef {
            index: self.0.index(),
            mty: MaterialType::Sponge,
        });
    }
    fn alloc(&self, curr_mty: &mut MaterialType, _alloc: &mut BufferAllocator) {
        *curr_mty = MaterialType::Sponge
    }
}
