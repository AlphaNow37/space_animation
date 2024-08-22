use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use bytemuck::{Pod, Zeroable};
use crate::utils::{VectorSpace, Zero};

/// A polynomial parametric curve
/// the eval is sum_(k=0)^(N-1) t^k * F_k
/// Same order as polynomials (t^n+...+t^0)
#[derive(Clone, Copy, PartialEq, Pod, Zeroable, Debug)]
#[repr(transparent)]
pub struct Curve<T, const N: usize>(pub [T; N]);
impl<T: VectorSpace, const N: usize> Curve<T, N> {
    pub fn from_bezier(points: [T; N]) -> Self {
        let mut mask = [0.; N];  // at the iteration n, the k element is k chosen in n: binomial(k, n)
        mask[0] = 1.;
        let mut binomial = 1.;  // ~ binomial(n, N)
        let mut coeffs = [T::ZERO; N];
        for (i, pt) in points.into_iter().rev().enumerate() {
            let pt_bin = pt * binomial;
            for j in 0..i+1 {
                coeffs[j] += pt_bin * mask[j];
            }

            binomial *= (N-1-i) as f32;
            binomial /= (i+1) as f32;

            if i+1 != N {
                for j in (1..i+2).rev() {
                    mask[j] = mask[j-1] - mask[j];
                }
                mask[0] = -mask[0];
            }
        }
        Self(coeffs)
    }

    pub fn eval(&self, t: f32) -> T {
        let mut res = self.0[0];
        for k in 1..N {
            res = res * t + self.0[k];
        }
        res
    }
}

macro_rules! impl_ops_self {
    (
        $(
            $tr: ident, $trassign: ident -> $method: ident, $methodassign: ident
        );* $(;)?
    ) => {
        $(
            impl<T: $tr<Output=T>+Copy, const N: usize> $tr for Curve<T, N> {
                type Output = Self;
                fn $method(mut self, rhs: Self) -> Self {
                    for i in 0..N {
                        self.0[i] = self.0[i].$method(rhs.0[i])
                    }
                    self
                }
            }
            impl<T: $trassign+Copy, const N: usize> $trassign for Curve<T, N> {
                fn $methodassign(&mut self, rhs: Self) {
                    for i in 0..N {
                        self.0[i].$methodassign(rhs.0[i])
                    }
                }
            }
        )*
    };
}
macro_rules! impl_ops_f32 {
    (
        $(
            $tr: ident, $trassign: ident -> $method: ident, $methodassign: ident
        );* $(;)?
    ) => {
        $(
            impl<T: $tr<f32, Output=T>, const N: usize> $tr<f32> for Curve<T, N> {
                type Output = Self;
                fn $method(self, rhs: f32) -> Self {
                    Self(self.0.map(|param| param.$method(rhs)))
                }
            }
            impl<T: $trassign<f32>, const N: usize> $trassign<f32> for Curve<T, N> {
                fn $methodassign(&mut self, rhs: f32) {
                    for param in &mut self.0 {
                        param.$methodassign(rhs)
                    }
                }
            }
        )*
    };
}
impl_ops_self!(
    Add, AddAssign -> add, add_assign;
    Sub, SubAssign -> sub, sub_assign;
);

impl_ops_f32!(
    Mul, MulAssign -> mul, mul_assign;
    Div, DivAssign -> div, div_assign;
);
impl<T: Neg<Output=T>, const N: usize> Neg for Curve<T, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.map(|param| param.neg()))
    }
}

impl<const N: usize> Curve<f32, N> {
    fn mul_vec<T: Mul<f32, Output=T>+Copy>(self, rhs: T) -> Curve<T, N> {
        Curve(self.0.map(|param| rhs*param))
    }
}

impl<T: Zero, const N: usize> Zero for Curve<T, N> {
    const ZERO: Self = Self([T::ZERO; N]);
}


#[test]
fn test() {
    dbg!(Curve::from_bezier([4., 7., 2.]));
}
