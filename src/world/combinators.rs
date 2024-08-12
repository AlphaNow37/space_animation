use std::ops::{Mul, Add};

use super::variator::{UpdateCtx, Variator};


pub struct Interpolate<A, B>(pub A, pub B);
impl<A: Variator, B: Variator<Item=A::Item>> Variator for Interpolate<A, B>
    where A::Item: Add<Output=A::Item> + Mul<f32, Output=A::Item>
{
    type Item = A::Item;
    fn update(&self, ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        self.0.update(ctx, world) * ctx.time + self.1.update(ctx, world) * (1.-ctx.time)
    }
}

pub struct TimeOffset<V>(pub V, pub f32);
impl<V: Variator> Variator for TimeOffset<V> {
    type Item = V::Item;
    fn update(&self, ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        self.0.update(UpdateCtx {
            time: ctx.time + self.1,
            ..ctx
        }, world)
    }
}

pub struct TimeMul<V>(pub V, pub f32);
impl<V: Variator> Variator for TimeMul<V> {
    type Item = V::Item;
    fn update(&self, ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        self.0.update(UpdateCtx {
            time: ctx.time * self.1,
            ..ctx
        }, world)
    }
}

pub struct TimeMod<V>(pub V, pub f32);
impl<V: Variator> Variator for TimeMod<V> {
    type Item = V::Item;
    fn update(&self, ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        self.0.update(UpdateCtx {
            time: ctx.time % self.1,
            ..ctx
        }, world)
    }
}
