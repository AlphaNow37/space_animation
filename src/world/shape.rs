use glam::Vec3A;

use super::{variator::{UpdateCtx, Variator}, world::World};

pub trait TriShape {
    fn size(&self) -> (usize, usize);
    fn put(&self, add_vertex: impl FnMut(Vec3A), add_index: impl FnMut(u32), ctx: UpdateCtx, world: &World);
}

pub struct Triangle<P1, P2, P3>(pub P1, pub P2, pub P3);
impl<P1: Variator<Item=Vec3A>, P2: Variator<Item=Vec3A>, P3: Variator<Item=Vec3A>> TriShape for Triangle<P1, P2, P3> {
    fn size(&self) -> (usize, usize) {
        (3, 3)
    }
    fn put(&self, mut add_vertex: impl FnMut(Vec3A), mut add_index: impl FnMut(u32), ctx: UpdateCtx, world: &World) {
        add_vertex(self.0.update(ctx, world));
        add_vertex(self.1.update(ctx, world));
        add_vertex(self.2.update(ctx, world));
        for idx in [0, 1, 2] {add_index(idx)};
    }
}
