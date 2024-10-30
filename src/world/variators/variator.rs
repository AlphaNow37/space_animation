use crate::utils::GeneralHash;
use crate::world::world::World;

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

    fn pipe<U>(self, f: impl Fn(Self::Item)->U + 'static) -> impl Variator<Item=U> where Self: Sized {
        move |world: &World| f(self.update(world))
    }
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
