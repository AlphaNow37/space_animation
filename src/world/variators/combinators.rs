// use core::f32;
use std::ops::Mul;
// use crate::math::ToAngle;

use crate::utils::VectorSpace;
use super::variator::{new_typed_variator, Variator};
use crate::world::world::World;


new_typed_variator!(
    [world],
    Interpolate(A, B: A::Item, T: f32) [A::Item: VectorSpace] => A::Item {A * (1.-T) + B * T}
);

macro_rules! var_modifier {
    (
        $world: ident,
        impl $trait_name: ident for $var: ident -> $ty: ty
        [
            $(
                $method_name: ident
                ($($arg_name: ident : $arg_ty: ty),* $(,)?)
                -> $struct_name: ident -> $out_ty: ty
                {$expr: expr}
            );* $(;)?
        ]
    ) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct $struct_name<V> {
                $var: V,
                $(
                    $arg_name: $arg_ty,
                )*
            }
            impl<V: Variator<Item=$ty>> Variator for $struct_name<V> {
                type Item = $out_ty;
                fn update(&self, $world: &World) -> Self::Item {
                    let Self {$var, $($arg_name),*} = self;
                    let $var = $var.update($world);
                    $expr
                }
                fn eq_var(&self, other: &Self) -> bool where Self: Sized {
                    self.$var.eq_var(&other.$var) $(
                        && other.$arg_name == self.$arg_name
                    )*
                }
                fn hash_var(&self) -> u32 {
                    let mut out = self.$var.hash_var();
                    $(
                        out = (out << 1) ^ self.$arg_name.hash_var();
                    )*
                    out
                }
            }
        )*
        pub trait $trait_name: Variator<Item=$ty> {
            $(
                fn $method_name(self, $($arg_name: $arg_ty),* ) -> $struct_name<Self> where Self: Sized {
                    $struct_name {$var: self, $($arg_name),*}
                }
            )*
        }
        impl<V: Variator<Item=$ty>> $trait_name for V {}
    };
}

// macro_rules! float_modifiers {
//     ($mname: ident, $sname: ident ($($ty: ty),* $(,)?): $v: ident, $arg: ident => $expr: expr) => {
//         #[derive(Clone, Copy, Debug, PartialEq)]
//         pub struct $sname<V>(pub V, pub ($($ty,)*));
//         crate::utils::make_trait_alias!(
//             $snale
//         )
//
//         // impl<V: Variator> Variator for $name<V> {
//         //     type Item = V::Item;
//         //     fn update(&self, ctx: super::variator::UpdateCtx, world: &crate::world::world::World) -> Self::Item {
//         //         let $arg = self.1;
//         //         self.0.update(UpdateCtx {
//         //             time: $expr,
//         //             ..ctx
//         //         }, world)
//         //     }
//         // }
//     };
// }
// time_modifier!(TimeOffset(f32): t,o => t+o.0);
// time_modifier!(TimeMul(f32): t,m => t*m.0);
// time_modifier!(TimeMod(f32): t,m => t%m.0);
// time_modifier!(TimeSin(f32): t,p => (t/p.0).turn().sin());
// time_modifier!(TimeLea(f32, f32): t,a => t.mul_add(a.0, a.1));

var_modifier!(
    world,
    impl FloatExt for v -> f32 [
        add(add: f32) -> AddF -> f32 {v + add};
        mul(mul: f32) -> MulF -> f32 {v * mul};
        lea(mul: f32, add: f32) -> LeaF -> f32 {v * mul + add};
        modulo(period: f32) -> ModuloF -> f32 {v.rem_euclid(*period)};
        sin(period: f32, amp: f32) -> SinF -> f32 {(v / period * std::f32::consts::TAU).sin() * amp};
    ]
);

pub struct MulV<A, B>(pub A, pub B);
impl<A: Variator, B: Variator> Variator for MulV<A, B> where A::Item: Mul<B::Item> {
    type Item = <A::Item as Mul<B::Item>>::Output;
    fn update(&self, world: &World) -> Self::Item {
        self.0.update(world) * self.1.update(world)
    }
}
