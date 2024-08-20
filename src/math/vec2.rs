use std::fmt::{Debug, Display, Formatter};
use std::simd::num::SimdFloat;
use std::simd::Simd;
use bytemuck::{Pod, Zeroable};
use crate::utils::impl_vector_space_simd;

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct Vec2(pub Simd<f32, 2>);
impl Vec2 {
    pub const X: Self = Self::new(1., 0.);
    pub const Y: Self = Self::new(0., 1.);
    pub const ONE: Self = Self::new(1., 1.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self(Simd::from_array([x, y]))
    }
    pub fn translate(self, other: Self) -> Self {
        self + other
    }
    pub fn len_squared(self) -> f32 {
        self.dot(self)
    }
    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }
    pub fn with_len_squared(self, len2: f32) -> Self {
        self * (len2 / self.len_squared()).sqrt()
    }
    pub fn with_len(self, len: f32) -> Self {
        self * len / self.len()
    }
    pub fn x(self) -> f32 {
        self.0[0]
    }
    pub fn y(self) -> f32 {
        self.0[1]
    }
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0).reduce_sum()
    }
    pub fn to_array(self) -> [f32; 2] {
        self.0.to_array()
    }
}
impl_vector_space_simd!(Vec2 (2));
// impl PartialEq for Vec3 {
//     fn eq(&self, other: &Self) -> bool {
//         simd_swizzle!(self.0, [0, 1, 2, 2]) == simd_swizzle!(other.0, [0, 1, 2, 2])
//     }
// }
impl Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {})", self.x(), self.y())
    }
}
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
