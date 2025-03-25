use crate::math::{Polynomial, Transform, Vec3};

use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::render_registry::vertex::VertexType;
use crate::world::visuals::VisualDirective;

use crate::world::world::Ref;

pub struct Triangle(pub Ref<Vec3>, pub Ref<Vec3>, pub Ref<Vec3>);
impl VisualDirective for Triangle {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_tri([self.0.index(), self.1.index(), self.2.index()])
    }
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::Tri, *curr_mty, 1);
    }
}
pub struct Sphere(pub Ref<Transform>);
impl VisualDirective for Sphere {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_sphere(self.0.index())
    }
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::Sphere, *curr_mty, 1);
    }
}

pub struct Cube(pub Ref<Transform>);
impl VisualDirective for Cube {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_cube(self.0.index())
    }
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::Cube, *curr_mty, 1);
    }
}

impl VisualDirective for Ref<Polynomial<Vec3, 4, 4>> {
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_poly4x4(self.index())
    }
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::Poly4x4, *curr_mty, 1);
    }
}

pub struct Tiled<T>(pub T, pub Ref<Transform>);
impl VisualDirective for Tiled<Triangle> {
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::TiledTri, *curr_mty, 1);
    }
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_tiled_tri(
            [self.0.0.index(), self.0.1.index(), self.0.2.index()],
            self.1.index(),
        )
    }
}

/// Pipe going in the z direction, from 0 to 1
pub struct Pipe(pub Ref<Transform>);
impl VisualDirective for Pipe {
    fn alloc(&self, curr_mty: &mut MaterialType, alloc: &mut BufferAllocator) {
        alloc.alloc_instance(VertexType::Pipe, *curr_mty, 1);
    }
    fn exec(&self, executor: &mut VisualExecutor) {
        executor.push_pipe(self.0.index())
    }
}

// pub struct Pyramid<A, S>(pub A, pub S);
// impl<A: TriShape+BorderShape, S: Variator<Item=Vec3>> TriShape for Pyramid<A, S> {
//     // const NB_INDEX: usize = A::NB_INDEX + 3*A::NB_BORDER_SEGMENT;
//     const NB_VERTEX: usize = A::NB_VERTEX + 3*A::NB_BORDER_SEGMENT;
//     // fn nb_vertex(&self) -> usize {
//     //     self.0.nb_vertex() + self.0.border_size()
//     // }
//     // fn nb_index(&self) -> usize {
//     //     self.0.nb_index() + self.0.border_size() * 3
//     // }
//     fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
//         let start = builder.next_index();
//         self.0.put(builder, ctx, world);
//         let top = self.1.update(ctx, world);
//         let top_pos = builder.global().tr_point(top);
//         for (i1, i2) in self.0.segment_border() {
//             put_triangle(builder, [
//                  builder.get_vertex(start+i1),
//                  builder.get_vertex(start+i2),
//                  TriVertex::create(top_pos, Normal::ZERO, top)
//             ])
//         }
//     }
// }
