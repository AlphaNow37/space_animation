use crate::math::Transform;
use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::world::world::Ref;

pub mod material;
pub mod shape;
pub use material::*;
pub use shape::*;

pub trait VisualDirective {
    fn exec(&self, executor: &mut VisualExecutor);
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator);
}

impl VisualDirective for Ref<Transform> {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.set_global(self.index());
    }
    fn alloc(&self, _curr_mty: &mut MaterialType, _alloc: &mut BufferAllocator) {}
}

macro_rules! impl_visual_dir_tuple {
    (
        $a: ident, $($t: ident, )*
    ) => {
        #[allow(non_snake_case, unused_variables)]
        impl<$($t: VisualDirective), *> VisualDirective for ($($t,)*) {
            fn exec(&self, executor: &mut VisualExecutor) {
                let ($($t, )*) = self;
                $(
                    $t.exec(executor);
                )*
            }
            fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
                let ($($t, )*) = self;
                $(
                    $t.alloc(curr_mty, alloc);
                )*
            }
        }
        impl_visual_dir_tuple!($($t, )*);
    };
    () => {}
}
impl_visual_dir_tuple!(A, B, C, D, E, F, G,);
