use super::{combinators::{TimeMod, TimeMul, TimeOffset}, world::World};

#[derive(Clone, Copy)]
pub struct UpdateCtx {
    pub time: f32,
}

pub trait Variator {
    type Item;
    fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item;

    fn time_mod(self, t: f32) -> TimeMod<Self> where Self: Sized {
        TimeMod(self, t)
    }
    fn time_add(self, t: f32) -> TimeOffset<Self> where Self: Sized {
        TimeOffset(self, t)
    }
    fn time_mul(self, t: f32) -> TimeMul<Self> where Self: Sized {
        TimeMul(self, t)
    }
}

macro_rules! impl_variator_tuple {
    ($a: ident $($t: ident)*) => {
        #[allow(non_snake_case, unused_variables)]
        impl<$($t: Variator, )*> Variator for ($($t, )*) {
            type Item = ($($t::Item, )*);
            fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item {
                let ($($t, )*) = self;
                ($(
                    $t.update(ctx, world),
                )*)
            }
        }
        impl_variator_tuple!($($t)*);
    };
    () => {};
}
// impl_variator_tuple!(AA AB BA BB CA CB DA DB EA EB);
impl_variator_tuple!(A B C D E F G);
