use glam::{Affine3A, Vec3A};

use super::{material::TriMeshBuilder, variator::{UpdateCtx, Variator}, world::World};

pub trait TriShape {
    fn nb_index(&self) -> usize;
    fn nb_vertex(&self) -> usize;
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World);
}

pub trait BorderShape {
    /// number of segment
    fn border_size(&self) -> usize;
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)>;
}
pub struct Triangle<P1, P2, P3>(pub P1, pub P2, pub P3);
impl<P1: Variator<Item=Vec3A>, P2: Variator<Item=Vec3A>, P3: Variator<Item=Vec3A>> TriShape for Triangle<P1, P2, P3> {
    fn nb_index(&self) -> usize {
        3
    }
    fn nb_vertex(&self) -> usize {
        3
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        builder.push_indexes_offset(&mut [0, 1, 2]);
        let p1 = self.0.update(ctx, world);
        let p2 = self.1.update(ctx, world);
        let p3 = self.2.update(ctx, world);
        let normal = (p1-p2).cross(p1-p3).normalize_or_zero();
        builder.push_vertexes(&[
            // (0; X; Y) if we truncate
            (p1, normal, Vec3A::Z),
            (p2, normal, Vec3A::X),
            (p3, normal, Vec3A::Y),
        ]);
    }
}
impl<P1, P2, P3> BorderShape for Triangle<P1, P2, P3> {
    fn border_size(&self) -> usize {
        3
    }
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)> {
        [(0, 1), (1, 2), (2, 0)]
    }
}

/// Cube, but with normals like a sphere. Have less verteces
pub struct CubeSphere<C>(pub C);
impl<C: Variator<Item=Affine3A>> TriShape for CubeSphere<C> {
    fn nb_index(&self) -> usize {
        36
    }
    fn nb_vertex(&self) -> usize {
        8
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let tr = self.0.update(ctx, world);
        builder.push_indexes_offset(&mut [
            0, 1, 2, 1, 2, 3, 
            0, 1, 4, 1, 4, 5,
            0, 2, 4, 2, 4, 6,
            1, 3, 5, 3, 5, 7,
            2, 3, 6, 3, 6, 7,
            4, 5, 6, 5, 6, 7,
        ]);
        for x in [-1., 1.] {
            for y in [-1., 1.] {
                for z in [-1., 1.] {
                    let uv = (Vec3A::new(x, y, z) + Vec3A::ONE) * 0.5;
                    let normal = tr.transform_vector3a(Vec3A::new(x, y, z));
                    let pos = normal + tr.translation;
                    builder.push_vertex((pos, normal.normalize(), uv));
                }
            }
        }
    }
}

pub struct Cube<C>(pub C);
impl<C: Variator<Item=Affine3A>> TriShape for Cube<C> {
    fn nb_index(&self) -> usize {
        36
    }
    fn nb_vertex(&self) -> usize {
        24
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let tr = self.0.update(ctx, world);

        for (normal, p, a, b) in [
            (tr.x_axis, Vec3A::ZERO, Vec3A::Y, Vec3A::Z),
            (tr.x_axis, Vec3A::X, Vec3A::Y, Vec3A::Z),
            (tr.y_axis, Vec3A::ZERO, Vec3A::X, Vec3A::Z),
            (tr.y_axis, Vec3A::Y, Vec3A::X, Vec3A::Z),
            (tr.z_axis, Vec3A::ZERO, Vec3A::X, Vec3A::Y),
            (tr.z_axis, Vec3A::Z, Vec3A::X, Vec3A::Y),
        ] {
            let normal = normal.normalize_or_zero();
            builder.push_indexes_offset(&mut [0, 1, 2, 1, 2, 3]);
            for uv in [p, p+a, p+b, p+a+b] {
                let pos = tr.transform_point3a(uv + uv - Vec3A::ONE);
                builder.push_vertex((pos, normal, uv));
            }
        }
    }
}

// pub struct Pyramid<A, S>(pub A, pub S);
// impl<A: TriShape+BorderShape, S: Variator<Item=Vec3A>> TriShape for Pyramid<A, S> {
//     fn nb_vertex(&self) -> usize {
//         self.0.nb_vertex() + self.0.border_size()
//     }
//     fn nb_index(&self) -> usize {
//         self.0.nb_index() + self.0.border_size() * 3
//     }
//     fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
//         let top = builder.push_vertex(self.1.update(ctx, world));
//         let next = builder.next_index();
//         for (a, b) in self.0.segment_border() {
//             builder.push_indexes(&[
//                 top,
//                 a+next,
//                 b+next,
//             ]);
//         }
//         self.0.put(builder, ctx, world);
//     }
// }
