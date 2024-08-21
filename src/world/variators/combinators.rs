use core::f32;
use std::ops::Mul;
use crate::math::ToAngle;

use crate::utils::VectorSpace;

use super::variator::{new_typed_variator, UpdateCtx, Variator};
use crate::world::world::World;


new_typed_variator!(
    [ctx, world],
    Interpolate(A, B: A::Item) [A::Item: VectorSpace] => A::Item {A * (1.-ctx.time) + B * ctx.time}
);

macro_rules! time_modifier {
    ($name: ident ($($ty: ty),* $(,)?): $time: ident, $arg: ident => $expr: expr) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name<V>(pub V, pub ($($ty,)*));
        impl<V: Variator> Variator for $name<V> {
            type Item = V::Item;
            fn update(&self, ctx: super::variator::UpdateCtx, world: &crate::world::world::World) -> Self::Item {
                let $time = ctx.time;
                let $arg = self.1;
                self.0.update(UpdateCtx {
                    time: $expr,
                    ..ctx
                }, world)
            }
        }
    };
}
time_modifier!(TimeOffset(f32): t,o => t+o.0);
time_modifier!(TimeMul(f32): t,m => t*m.0);
time_modifier!(TimeMod(f32): t,m => t%m.0);
time_modifier!(TimeSin(f32): t,p => (t/p.0).turn().sin());
time_modifier!(TimeLea(f32, f32): t,a => t.mul_add(a.0, a.1));


pub struct MulV<A, B>(pub A, pub B);
impl<A: Variator, B: Variator> Variator for MulV<A, B> where A::Item: Mul<B::Item> {
    type Item = <A::Item as Mul<B::Item>>::Output;
    fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item {
        self.0.update(ctx, world) * self.1.update(ctx, world)
    }
}
