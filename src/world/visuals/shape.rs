use crate::math::{Transform, Vec3};

use crate::render_registry::vertex::{Normal, TriVertex};

use crate::world::{visuals::mesh_builder::TriMeshBuilder, variators::variator::{UpdateCtx, Variator}, world::World};

pub trait TriShape {
    const NB_INDEX: usize;
    const NB_VERTEX: usize;

    // /// Cheap
    // fn nb_index(&self) -> usize;
    // /// Cheap
    // fn nb_vertex(&self) -> usize;
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World);
}

pub trait BorderShape {
    const NB_BORDER_SEGMENT: usize;
    // /// number of segment
    // fn border_size(&self) -> usize;
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)>;
}
// Overwrite normals
fn put_triangle(builder: &mut impl TriMeshBuilder, mut ps: [TriVertex; 3]) {
    let normal = Normal::from_tri(ps[0].pos, ps[1].pos, ps[2].pos);
    for p in &mut ps {
        p.normal = normal;
    }
    builder.push_indexes_offset([0, 1, 2]);
    builder.push_vertexes(ps)
}

pub struct Triangle<P1, P2, P3>(pub P1, pub P2, pub P3);
impl<P1: Variator<Item=Vec3>, P2: Variator<Item=Vec3>, P3: Variator<Item=Vec3>> TriShape for Triangle<P1, P2, P3> {
    const NB_INDEX: usize = 3;
    const NB_VERTEX: usize = 3;
    // fn nb_index(&self) -> usize {
    //     3
    // }
    // fn nb_vertex(&self) -> usize {
    //     3
    // }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let (a, b, c) = (self.0.update(ctx, world), self.1.update(ctx, world), self.2.update(ctx, world));
        let g = builder.global();
        let (apos, bpos, cpos) = (g.tr_point(a), g.tr_point(b), g.tr_point(c));
        put_triangle(builder, [
            TriVertex::new(apos,  Normal::ZERO, a),
            TriVertex::new(bpos, Normal::ZERO, b),
            TriVertex::new(cpos, Normal::ZERO, c),
        ])
    }
}
impl<P1: Variator<Item=Vec3>, P2: Variator<Item=Vec3>, P3: Variator<Item=Vec3>> BorderShape for Triangle<P1, P2, P3> {
    const NB_BORDER_SEGMENT: usize = 3;
    // fn border_size(&self) -> usize {
    //     3
    // }
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)> {
        [(0, 1), (1, 2), (2, 0)]
    }
}

/// Cube, but with normals like a sphere. Have fewer vertexes
/// Half extents
pub struct CubeSphere<C>(pub C);
impl<C: Variator<Item=Transform>> TriShape for CubeSphere<C> {
    const NB_INDEX: usize = 36;
    const NB_VERTEX: usize = 8;
    // fn nb_index(&self) -> usize {
    //     36
    // }
    // fn nb_vertex(&self) -> usize {
    //     8
    // }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let tr = self.0.update(ctx, world);
        builder.push_indexes_offset([
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
                    let normal = tr.tr_vec(Vec3::new(x, y, z));
                    let uv = normal + tr.trans();
                    let pos = builder.global().tr_point(uv);
                    builder.push_vertex(TriVertex::new(pos, normal.into(), uv))
                }
            }
        }
    }
}

pub struct Cube<C>(pub C);
impl<C: Variator<Item=Transform>> TriShape for Cube<C> {
    const NB_INDEX: usize = 36;
    const NB_VERTEX: usize = 24;
    // fn nb_index(&self) -> usize {
    //     36
    // }
    // fn nb_vertex(&self) -> usize {
    //     24
    // }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let tr = self.0.update(ctx, world);
        for (normal, p, a, b) in [
            (tr.x(), Vec3::ZERO, Vec3::Y, Vec3::Z),
            (tr.x(), Vec3::X, Vec3::Y, Vec3::Z),
            (tr.y(), Vec3::ZERO, Vec3::X, Vec3::Z),
            (tr.y(), Vec3::Y, Vec3::X, Vec3::Z),
            (tr.z(), Vec3::ZERO, Vec3::X, Vec3::Y),
            (tr.z(), Vec3::Z, Vec3::X, Vec3::Y),
        ] {
            let normal: Normal = builder.global().tr_vec(normal).into();
            builder.push_indexes_offset([0, 1, 2, 1, 2, 3]);
            for coord in [p, p+a, p+b, p+a+b] {
                let uv = tr.tr_point(coord);
                let pos = builder.global().tr_point(uv);
                builder.push_vertex(TriVertex::new(pos, normal, uv));
            }
        }
    }
}

pub struct Pyramid<A, S>(pub A, pub S);
impl<A: TriShape+BorderShape, S: Variator<Item=Vec3>> TriShape for Pyramid<A, S> {
    const NB_INDEX: usize = A::NB_INDEX + 3*A::NB_BORDER_SEGMENT;
    const NB_VERTEX: usize = A::NB_VERTEX + 3*A::NB_BORDER_SEGMENT;
    // fn nb_vertex(&self) -> usize {
    //     self.0.nb_vertex() + self.0.border_size()
    // }
    // fn nb_index(&self) -> usize {
    //     self.0.nb_index() + self.0.border_size() * 3
    // }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let start = builder.next_index();
        self.0.put(builder, ctx, world);
        let top = self.1.update(ctx, world);
        let top_pos = builder.global().tr_point(top);
        for (i1, i2) in self.0.segment_border() {
            put_triangle(builder, [
                 builder.get_vertex(start+i1),
                 builder.get_vertex(start+i2), 
                 TriVertex::new(top_pos, Normal::ZERO, top)
            ])
        }
    }
}
