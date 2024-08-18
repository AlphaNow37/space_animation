// use std::ops::{Add, Mul};

use glam::{Affine3A, Mat3A, Mat4, Quat, Vec2, Vec3A, Vec4};

use crate::world::color::Color;


// Can't use generic because of other impls...
macro_rules! impl_vect_space {
    (
        $($ty: ty),* $(,)?
    ) => {
        $(
            impl VectorSpace for $ty {
                fn mul(self, t: f32) -> Self {
                    self * t
                }
                fn add(self, other: Self) -> Self {
                    self + other
                }
            }
        )*
    };
}
macro_rules! impl_vect_space_decomp {
    (
        $(
            $ty: ty {
                $(
                    $attr: ident
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        $(
            impl VectorSpace for $ty {
                fn mul(self, t: f32) -> Self {
                    Self {
                        $(
                            $attr: self.$attr.mul(t),
                        )*
                    }
                }
                fn add(self, other: Self) -> Self {
                    Self {
                        $(
                            $attr: self.$attr.add(other.$attr),
                        )*
                    }
                }
            }
        )*
    };
}

impl_vect_space!(
    f32, Vec2, Vec3A, Color, Vec4, Mat3A, Mat4, Quat
);
impl_vect_space_decomp!(
    Affine3A {matrix3, translation}
);

pub trait VectorSpace {
    fn mul(self, t: f32) -> Self;
    fn add(self, other: Self) -> Self;
}
// impl<T: Add<Self, Output=T> + Mul<f32, Output=T>> VectorSpace for T {
//     fn add(self, other: Self) -> Self {
//         <Self as Add>::add(self, other)
//     }
//     fn mul(self, t: f32) -> Self {
//         <Self as Mul<f32>>::mul(self, t)
//     }
// }
