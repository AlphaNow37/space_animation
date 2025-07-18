use super::super::{primitives::color::Color, variators::variator::Variator};
use crate::math::{Vec2, Vec3, Vec4};

macro_rules! make_pack {
    (
        $(
            $sname: ident <$($gen: ident),*> => $res: ident
        );*
        $(;)?
    ) => {
        $(
            pub struct $sname<$($gen),*>($(pub $gen),*);
            impl<$($gen: Variator<Item=f32>),*> Variator for $sname<$($gen),*> {
                type Item=$res;
                #[allow(non_snake_case)]
                fn update(&self, worlds: &crate::world::world::Worlds) -> Self::Item {
                    let $sname($($gen),*) = self;
                    $res::new(
                        $(
                            $gen.update(worlds)
                        ),*
                    )
                }
            }
        )*
    };
}

make_pack!(
    Pack2<A, B> => Vec2;
    Pack3<A, B, C> => Vec3;
    Pack4<A, B, C, D> => Vec4;
    PackCol<A, B, C> => Color;
);
