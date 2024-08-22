use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use crate::utils::make_trait_alias;
use crate::world::primitives::color::Color;

pub trait Zero {
    const ZERO: Self;
}
impl Zero for f32 {
    const ZERO: Self = 0.;
}

make_trait_alias!(
    VectorSpace = [
        Sized
        + Add<Output=Self>
        + AddAssign
        + Sub<Output=Self>
        + SubAssign
        + Mul<f32, Output=Self>
        + MulAssign<f32>
        + Div<f32, Output=Self>
        + DivAssign<f32>
        + Copy
        + Zero
    ] {
        fn mid(self, other: Self) -> Self {
            (self + other) * 0.5
        }
    }
);

// pub trait VectorSpace: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero {
//     fn mid(self, other: Self) -> Self {
//         (self + other) * 0.5
//     }
// }
// impl<T: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero> VectorSpace for T {}
