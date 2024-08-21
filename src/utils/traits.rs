use std::ops::{Add, Sub, Mul, Div};
use crate::world::primitives::color::Color;

pub trait VectorSpace: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> {
    fn mid(self, other: Self) -> Self {
        (self + other) * 0.5
    }
}
impl<T: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self>> VectorSpace for T {}
