#![allow(const_item_mutation)]

use glam::{Affine3A, Vec3A};

use super::{material::TriMeshBuilder, variator::{UpdateCtx, Variator}, world::World};



const CUBE_INDEX: [u32; 36] = [
    0, 1, 2, 1, 2, 3, 
    0, 1, 4, 1, 4, 5,
    0, 2, 4, 2, 4, 6,
    1, 3, 5, 3, 5, 7,
    2, 3, 6, 3, 6, 7,
    4, 5, 6, 5, 6, 7,
];

pub trait TriShape {
    fn size(&self) -> (usize, usize);
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World);
}

pub trait BorderShape {
    fn border_sizes(&self) -> impl AsRef<[usize]>;
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)>;
}
pub struct Triangle<P1, P2, P3>(pub P1, pub P2, pub P3);
impl<P1: Variator<Item=Vec3A>, P2: Variator<Item=Vec3A>, P3: Variator<Item=Vec3A>> TriShape for Triangle<P1, P2, P3> {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        builder.push_indexes_offset(&mut [0, 1, 2]);
        builder.push_vertexes(&[
            self.0.update(ctx, world),
            self.1.update(ctx, world),
            self.2.update(ctx, world),
        ]);
    }
}
impl<P1, P2, P3> BorderShape for Triangle<P1, P2, P3> {
    fn border_sizes(&self) -> impl AsRef<[usize]> {
        &[3]
    }
    fn segment_border(&self) -> impl IntoIterator<Item=(u32, u32)> {
        [(0, 1), (1, 2), (2, 0)]
    }
}

pub struct Cube<C>(pub C);
impl<C: Variator<Item=Affine3A>> TriShape for Cube<C> {
    fn size(&self) -> (usize, usize) {
        (8, 36)
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let tr = self.0.update(ctx, world);
        for i in 0..8 {
            let x = [0., 1.][i & 1];
            let y = [0., 1.][i & 2];
            let z = [0., 1.][i & 4];
            builder.push_vertex(tr.transform_point3a(Vec3A::new(x, y, z)));
        }
        builder.push_indexes_offset(&mut CUBE_INDEX);
    }
}

pub struct Pyramid<A, S>(pub A, pub S);
impl<A: TriShape+BorderShape, S: Variator<Item=Vec3A>> TriShape for Pyramid<A, S> {
    fn size(&self) -> (usize, usize) {
        let (vs, is) = self.0.size();
        let add_index = self.0.border_sizes()
            .as_ref()
            .into_iter()
            .map(|n| match n {
                0 | 1 => 0,
                n => 3*n,
            })
            .sum::<usize>();
        (vs + 1, is + add_index)
    }
    fn put(&self, builder: &mut impl TriMeshBuilder, ctx: UpdateCtx, world: &World) {
        let top = builder.push_vertex(self.1.update(ctx, world));
        for (a, b) in self.0.segment_border() {
            builder.push_indexes(&[
                top,
                a+builder.next_index(),
                b+builder.next_index(),
            ]);
        }
        self.0.put(builder, ctx, world);
    }
}
