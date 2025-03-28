macro_rules! array_key {
    (
        $vis: vis
        enum
        $name: ident
        {
            $($variant: ident),*
            $(,)?
        }
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        $vis enum $name {
            $($variant),*
        }
        impl $name {
            #[allow(dead_code, path_statements)]
            $vis const COUNT: usize = $({Self::$variant; 1} + )* 0;
            #[allow(dead_code)]
            $vis const ARRAY: [Self; Self::COUNT] = [$(Self::$variant),*];

            #[allow(dead_code)]
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant)
                    ),*
                }
            }
        }
    };
}
pub(crate) use array_key;

macro_rules! impl_vector_space_simd {
    (
        $t: ident ($n: literal)
    ) => {
        use crate::utils::Zero;
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
        use std::simd::ToBytes;

        impl $t {
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.simd_clamp(min.0, max.0))
            }
        }
        impl Zero for $t {
            const ZERO: Self = Self(Simd::from_array([0.; $n]));
        }
        impl Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }
        impl Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }
        impl Mul<f32> for $t {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self::Output {
                Self(self.0 * Simd::splat(rhs))
            }
        }
        impl MulAssign<f32> for $t {
            fn mul_assign(&mut self, rhs: f32) {
                self.0 *= Simd::splat(rhs)
            }
        }
        impl Div<f32> for $t {
            type Output = Self;
            fn div(self, rhs: f32) -> Self::Output {
                Self(self.0 / Simd::splat(rhs))
            }
        }
        impl DivAssign<f32> for $t {
            fn div_assign(&mut self, rhs: f32) {
                self.0 /= Simd::splat(rhs)
            }
        }
        impl Neg for $t {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }
        impl Default for $t {
            fn default() -> Self {
                Self::ZERO
            }
        }
        impl std::hash::Hash for $t {
            fn hash<T: std::hash::Hasher>(&self, hasher: &mut T) {
                self.0.to_ne_bytes().hash(hasher)
            }
        }
    };
}
pub(crate) use impl_vector_space_simd;

macro_rules! make_trait_alias {
    (
        $new: ident = [$($old: tt)*] { $($content: tt)* }
    ) => {
        pub trait $new: $($old)* { $($content)* }
        impl<T: $($old)*> $new for T {}
    };
}
pub(crate) use make_trait_alias;

// macro_rules! forget {
//     (
//         $($i: tt)*
//     ) => {
//
//     };
// }
// pub(crate) use forget;

macro_rules! make_deref {
    ($s: ty, $t: ty) => {
        impl std::ops::Deref for $s {
            type Target = $t;
            fn deref(&self) -> Self::Target {
                self.0
            }
        }
    };
}
pub(crate) use make_deref;
