use crate::world::world::World;
use crate::world::variators::combinators::{TimeLea, TimeMod, TimeMul, TimeOffset, TimeSin};

#[derive(Clone, Copy)]
pub struct UpdateCtx {
    pub time: f32,
}

macro_rules! time_methods {
    (
        $($name: ident ($($arg: ident : $ty: ty),* $(,)?)=>$s: ident);* $(;)?
    ) => {
        $(
            fn $name(self, $($arg: $ty,)*) -> $s<Self> where Self: Sized {
                $s(self, ($($arg,)*))
            }
        )*
    };
}

#[allow(dead_code)]
pub trait Variator: 'static {
    type Item;
    fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item;

    time_methods!(
        time_mod(t: f32) => TimeMod;
        time_add(t: f32) => TimeOffset;
        time_mul(t: f32) => TimeMul;
        time_sin(p: f32) => TimeSin;
        time_lea(m: f32, a: f32) => TimeLea;
    );
}

// macro_rules! impl_variator_tuple {
//     ($a: ident $($t: ident)*) => {
//         #[allow(non_snake_case, unused_variables)]
//         impl<$($t: Variator, )*> Variator for ($($t, )*) {
//             type Item = ($($t::Item, )*);
//             fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item {
//                 let ($($t, )*) = self;
//                 ($(
//                     $t.update(ctx, world),
//                 )*)
//             }
//         }
//         impl_variator_tuple!($($t)*);
//     };
//     () => {};
// }
// // impl_variator_tuple!(AA AB BA BB CA CB DA DB EA EB);
// impl_variator_tuple!(A B C D E F G);

macro_rules! new_typed_variator {
    (
        $([$ctx: ident, $world: ident],)?
        $name: ident ($($gen: ident $(: $ty: ty)?),* $(,)?) $([$($clause: tt)*])? => $out: ty {$($ins: tt)*} 
    ) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name<$($gen),*>($(pub $gen),*);
        #[allow(non_snake_case, dead_code, unused_variables)]
        impl<$($gen: Variator$(<Item=$ty>)?),*> crate::world::variators::variator::Variator for $name<$($gen),*> $(where $($clause)*)? {
            type Item=$out;
            fn update(&self, ctx: crate::world::variators::variator::UpdateCtx, world: &crate::world::world::World) -> Self::Item {
                let $name($($gen, )*) = self;
                $(
                    let $gen = $gen.update(ctx, world);
                )*
                $(
                    let $ctx = ctx;
                    let $world = world;
                )?
                $($ins)*
            }
        }
    };
}

pub(crate) use new_typed_variator;

impl<U, T: (Fn(UpdateCtx, &World)->U)+'static> Variator for T {
    type Item = U;
    fn update(&self, ctx: UpdateCtx, world: &World) -> Self::Item {
        self(ctx, world)
    }
}

// macro_rules! new_untyped_variator {
//     (
//         $name: ident ($($gen: ident),* $(,)?) $([$($clause: tt)*])? => $out: ty {$($ins: tt)*} 
//     ) => {
//         #[allow(dead_code)]
//         #[derive(Clone, Copy, Debug, PartialEq)]
//         pub struct $name<$($gen),*>($(pub $gen),*);
//         #[allow(non_snake_case, dead_code, unused_variables)]
//         impl<$($gen: Variator),*> crate::world::variator::Variator for $name<$($gen),*> $(where $($clause)*)? {
//             type Item=$out;
//             fn update(&self, ctx: crate::world::variator::UpdateCtx, world: &crate::world::world::World) -> Self::Item {
//                 let $name($($gen, )*) = self;
//                 $(
//                     let $gen = $gen.update(ctx, world);
//                 )*
//                 $($ins)*
//             }
//         }
//     };
// }
// pub(crate) use new_untyped_variator;
