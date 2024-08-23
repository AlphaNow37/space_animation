use std::array::from_fn;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use bytemuck::{Pod, Zeroable};
use crate::utils::{binomial, VectorSpace, Zero};

/// A polynomial parametric curve
/// the eval is sum_(k=0)^(N-1) t^k * F_k
/// Inverse order as polynomials (t^0+...+t^n)
#[derive(Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct Polynomial<T, const N: usize, const M: usize>(pub [[T; N]; M]);
impl<T: VectorSpace, const N: usize> Polynomial<T, N, 1> {
    pub fn new_bezier_curve(points: [T; N]) -> Self {  // Can't be const because of the non-const T*f32 & T+T
        let mut coeffs = [T::ZERO; N];
        let mut k = 0;
        while k < N {
            let pt = points[N-1-k] * binomial(k, N-1) as f32;
            let mut j = 0;
            while j <= k {
                coeffs[N-1-j] += pt * binomial(j, k) as f32 * [1., -1.][(k +j) % 2];
                j += 1;
            }
            k += 1;
        }
        Self([coeffs])
    }
    pub fn derivative(&self) -> Self {
        self.derivative_x()
    }
    pub fn eval_curve(&self, t: f32) -> T {
        let mut res = self.0[0][N-1];
        for k in 1..N {
            res = res * t + self.0[0][N-1-k];
        }
        res
    }
}
impl<T: VectorSpace, const N: usize, const M: usize> Polynomial<T, N, M> {
    pub fn new_bezier_surface(points: [[T; N]; M]) -> Self {  // Can't be const because of the non-const T*f32 & T+T
        let mut coeffs = [[T::ZERO; N]; M];

        for k1 in 0..M {
            for k2 in 0..N {
                let pt = points[M-1-k1][N-1-k2] * (binomial(k1, M-1) * binomial(k2, N-1)) as f32;
                for j1 in 0..=k1 {
                    for j2 in 0..=k2 {
                        coeffs[M-1-j1][N-1-j2] += pt * (binomial(j1, k1)*binomial(j2, k2)) as f32 * [1., -1.][(k1+k2+j1+j2) % 2];
                    }
                }
            }
        }
        Self(coeffs)
    }
    pub fn derivative_x(mut self) -> Self {
        for row in &mut self.0 {
            for x in 1..N {
                row[x-1] = row[x] * x as f32;
            }
            row[N-1] = T::ZERO;
        }
        self
    }
    pub fn derivative_y(mut self) -> Self {
        for y in 1..M {
            for x in 0..N {
                self.0[y-1][x] = self.0[y][x] * y as f32;
            }
        }
        self.0[M-1] = [T::ZERO; N];
        self
    }
    pub fn eval_surface(&self, t1: f32, t2: f32) -> T {
        let mut res = Polynomial([self.0[0]]).eval_curve(t1);
        for k in 1..M {
            res = res * t2 + Polynomial([self.0[k]]).eval_curve(t1)
        }
        res
    }
    pub fn to_size<const N2: usize, const M2: usize>(&self) -> Polynomial<T, N2, M2> {
        Polynomial(
            from_fn(|y|
                self.0.get(y)
                    .map_or([T::ZERO; N2], |row|
                        from_fn(|x|
                            row.get(x).copied().unwrap_or(T::ZERO)
                        )
                    )
            )
        )
    }
}

impl<T: Debug, const N: usize, const M: usize> Debug for Polynomial<T, N, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if M == 1 {
            f.write_str("Polynomial([")?;
            self.0[0].fmt(f)?;
            f.write_str("])")?;
        } else {
            f.write_str("Polynomial(")?;
            self.0.fmt(f)?;
            f.write_str(")")?;
        }
        Ok(())
    }
}

macro_rules! impl_ops_self {
    (
        $(
            $tr: ident, $trassign: ident -> $method: ident, $methodassign: ident
        );* $(;)?
    ) => {
        $(
            impl<T: $tr<Output=T>+Copy, const N: usize, const M: usize> $tr for Polynomial<T, N, M> {
                type Output = Self;
                fn $method(mut self, rhs: Self) -> Self {
                    for i in 0..M {
                        for j in 0..N {
                            self.0[i][j] = self.0[i][j].$method(rhs.0[i][j])
                        }
                    }
                    self
                }
            }
            impl<T: $trassign+Copy, const N: usize, const M: usize> $trassign for Polynomial<T, N, M> {
                fn $methodassign(&mut self, rhs: Self) {
                    for i in 0..M {
                        for j in 0..N {
                            self.0[i][j].$methodassign(rhs.0[i][j])
                        }
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
            impl<T: $tr<f32, Output=T>, const N: usize, const M: usize> $tr<f32> for Polynomial<T, N, M> {
                type Output = Self;
                fn $method(self, rhs: f32) -> Self {
                    Self(self.0.map(|row| row.map(|param| param.$method(rhs))))
                }
            }
            impl<T: $trassign<f32>, const N: usize, const M: usize> $trassign<f32> for Polynomial<T, N, M> {
                fn $methodassign(&mut self, rhs: f32) {
                    for row in &mut self.0 {
                        for param in row {
                            param.$methodassign(rhs)
                        }
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
impl<T: Neg<Output=T>, const N: usize, const M: usize> Neg for Polynomial<T, N, M> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.map(|row| row.map(Neg::neg)))
    }
}

impl<const N: usize, const M: usize> Polynomial<f32, N, M> {
    fn mul_vec<T: Mul<f32, Output=T>+Copy>(self, rhs: T) -> Polynomial<T, N, M> {
        Polynomial(self.0.map(|row| row.map(|f| rhs * f)))
    }
}

impl<T: Zero + Copy, const N: usize, const M: usize> Zero for Polynomial<T, N, M> {
    const ZERO: Self = Self([[T::ZERO; N]; M]);
}
impl<T: Zero + Copy, const N: usize, const M: usize> Default for Polynomial<T, N, M> {
    fn default() -> Self {
        Self::ZERO
    }
}

#[test]
fn test() {
    dbg!(Polynomial::new_bezier_curve([4., 7., 2.]));
    let p = dbg!(Polynomial::new_bezier_surface(
        [
            [1., 2., 5.],
            [3., -1., 4.],
        ]
    ));
    dbg!(p.eval_surface(0.5, 0.5));
    let dx = dbg!(p.derivative_x());
    dbg!(dx.eval_surface(0.5, 0.5));
    let dy = dbg!(p.derivative_y());
    dbg!(dy.eval_surface(0.5, 0.5));
}