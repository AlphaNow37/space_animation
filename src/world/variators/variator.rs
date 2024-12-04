use crate::utils::GeneralHash;
use crate::world::world::World;
// use crate::world::variators::combinators::{TimeLea, TimeMod, TimeMul, TimeOffset, TimeSin};

// #[derive(Clone, Copy)]
// pub struct UpdateCtx {
//     pub time: f32,
// }

// macro_rules! time_methods {
//     (
//         $($name: ident ($($arg: ident : $ty: ty),* $(,)?)=>$s: ident);* $(;)?
//     ) => {
//         $(
//             fn $name(self, $($arg: $ty,)*) -> $s<Self> where Self: Sized {
//                 $s(self, ($($arg,)*))
//             }
//         )*
//     };
// }

#[allow(dead_code)]
pub trait Variator: 'static + std::any::Any {
    type Item;
    fn update(&self, world: &World) -> Self::Item;

    #[allow(unused_variables)]
    fn hash_var(&self) -> u32 {1}
    fn finished_hash_var(&self) -> u32 {
        (self.hash_var(), self.type_id()).gen_hash()
    }
    #[allow(unused_variables)]
    fn eq_var(&self, other: &Self) -> bool where Self: Sized {false}

    // time_methods!(
    //     time_mod(t: f32) => TimeMod;
    //     time_add(t: f32) => TimeOffset;
    //     time_mul(t: f32) => TimeMul;
    //     time_sin(p: f32) => TimeSin;
    //     time_lea(m: f32, a: f32) => TimeLea;
    // );
}

macro_rules! new_typed_variator {
    (
        $([$world: ident],)?
        $name: ident ($($gen: ident $(: $ty: ty)?),* $(,)?) $([$($clause: tt)*])? => $out: ty {$($ins: tt)*} 
    ) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $name<$($gen),*>($(pub $gen),*);
        #[allow(non_snake_case, dead_code, unused_variables)]
        impl<$($gen: Variator$(<Item=$ty>)?),*> crate::world::variators::variator::Variator for $name<$($gen),*> $(where $($clause)*)? {
            type Item=$out;
            fn update(&self, world: &crate::world::world::World) -> Self::Item {
                let $name($($gen, )*) = self;
                $(
                    let $gen = $gen.update(world);
                )*
                $(
                    let $world = world;
                )?
                $($ins)*
            }
            fn hash_var(&self) -> u32 {
                let $name($($gen, )*) = self;
                let mut out = 0;
                $(
                    out = (out << 1) ^ $gen.hash_var();
                )*
                out
            }
            fn eq_var(&self, other: &Self) -> bool {
                struct S<$($gen, )*> {$($gen: $gen, )*}
                let $name($($gen, )*) = other;
                let s = S {$($gen, )*};
                let $name($($gen, )*) = self;
                $(
                    $gen.eq_var(s.$gen) &&
                )* true
            }
        }
    };
}

pub(crate) use new_typed_variator;

impl<U, T: (Fn(&World)->U)+'static> Variator for T {
    type Item = U;
    fn update(&self, world: &World) -> Self::Item {
        self(world)
    }
}
