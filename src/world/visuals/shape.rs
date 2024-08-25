use crate::math::{Transform, Vec3};

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

// impl<P1: Variator<Item=Vec3>, P2: Variator<Item=Vec3>, P3: Variator<Item=Vec3>> BorderShape for Triangle<P1, P2, P3> {
//     const NB_BORDER_SEGMENT: usize = 3;
//     // fn border_size(&self) -> usize {
//     //     3
//     // }
//     fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)> {
//         [(0, 1), (1, 2), (2, 0)]
//     }
// }

// /// Cube, but with normals like a sphere. Have fewer vertexes
// /// Half extents
// pub struct CubeSphere<C>(pub C);
// impl<C: Variator<Item=Transform>> TriShape for CubeSphere<C> {
//     // const NB_INDEX: usize = 36;
//     const NB_VERTEX: usize = 8;
//     // fn nb_index(&self) -> usize {
//     //     36
//     // }
//     // fn nb_vertex(&self) -> usize {
//     //     8
//     // }
//     fn put(&self, builder: &mut impl TriMeshBuilder, global: usize, mat: usize) {
//         let tr = self.0.update(ctx, world);
//         builder.push_indexes_offset([
//             0, 1, 2, 1, 2, 3,
//             0, 1, 4, 1, 4, 5,
//             0, 2, 4, 2, 4, 6,
//             1, 3, 5, 3, 5, 7,
//             2, 3, 6, 3, 6, 7,
//             4, 5, 6, 5, 6, 7,
//         ]);
//         for x in [-1., 1.] {
//             for y in [-1., 1.] {
//                 for z in [-1., 1.] {
//                     let normal = tr.tr_vec(Vec3::new(x, y, z));
//                     let uv = normal + tr.trans();
//                     let pos = builder.global().tr_point(uv);
//                     builder.push_vertex(TriVertex::create(pos, normal, uv))
//                 }
//             }
//         }
//     }
// }

// pub struct Cube<C>(pub C);
// impl<C: Variator<Item=Transform>> TriShape for Cube<C> {
//     // const NB_INDEX: usize = 36;
//     const NB_VERTEX: usize = 24;
//     // fn nb_index(&self) -> usize {
//     //     36
//     // }
//     // fn nb_vertex(&self) -> usize {
//     //     24
//     // }
//     fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
//         let tr = self.0.update(ctx, world);
//         for (normal, p, a, b) in [
//             (tr.x(), vec3(-1., -1., -1.), Vec3::Y, Vec3::Z),
//             (tr.x(), vec3(1., -1., -1.), Vec3::Y, Vec3::Z),
//             (tr.y(), vec3(-1., -1., -1.), Vec3::X, Vec3::Z),
//             (tr.y(), vec3(-1., 1., -1.), Vec3::X, Vec3::Z),
//             (tr.z(), vec3(-1., -1., -1.), Vec3::X, Vec3::Y),
//             (tr.z(), vec3(-1., -1., 1.), Vec3::X, Vec3::Y),
//         ] {
//             let normal: Normal = builder.global().tr_vec(normal).into();
//             builder.push_indexes_offset([0, 1, 2, 1, 2, 3]);
//             for coord in [p, p+a*2., p+b*2., p+a*2.+b*2.] {
//                 let uv = tr.tr_point(coord);
//                 let pos = builder.global().tr_point(uv);
//                 builder.push_vertex(TriVertex::create(pos, normal, uv));
//             }
//         }
//     }
// }

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
