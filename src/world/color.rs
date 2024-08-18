use std::ops::{Add, Mul};

use glam::Vec3A;

macro_rules! consts_rgb {
    (
        $($name: ident = $r: expr, $g: expr, $b: expr);*
        $(;)?
    ) => {
        impl Color {
            $(
                pub const $name: Self = Self::from_rgb($r, $g, $b);
            )*
        }
    };
}

/// OKLAB color
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Color(Vec3A);
impl Color {
    pub fn as_u32(self) -> u32 {
        u32::from_le_bytes(self.0.extend(1.).to_array().map(|c| c.clamp(0., 1.).mul(255.) as u8))
    }
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        // let vec = Vec3A::new(r, g, b);
        // glam::Mat3A::mul_vec3a(&self, rhs)
        Self(Vec3A::new(r, g, b)) // TODO: oklab
    }
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self(Vec3A::new(l, a, b))
    }
}
consts_rgb!(
    WHITE = 1., 1., 1.;
    BLACK = 0., 0., 0.;
    DEBUG = 0.8, 0., 0.4;
    RED = 1., 0., 0.;
    GREEN = 0., 1., 0.;
    BLUE = 0., 0., 1.;
);
impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
