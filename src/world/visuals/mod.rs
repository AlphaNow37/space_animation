use crate::math::Transform;
use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::world::world::Ref;

pub mod material;
pub mod shape;
pub use shape::*;
pub use material::*;

pub trait VisualDirective {
    fn exec(&self, executor: &mut VisualExecutor);
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator);
}
pub struct SetGlobal(pub Ref<Transform>);
impl VisualDirective for SetGlobal {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_global(self.0.index())
    }
    fn alloc(&self, _curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {}
}
