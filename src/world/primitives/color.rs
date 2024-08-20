use std::ops::{Add, Mul};
use crate::math::{Vec3, vec3};

use crate::utils::{compress_vec4_i, CompressedVec};

macro_rules! consts_lch {
    (
        $($name: ident = $l: expr, $c: expr, $h: expr);*
        $(;)?
    ) => {
        impl Color {
            $(
                #[allow(dead_code)]
                pub const $name: Self = Self::from_oklchf($l, $c, $h*std::f32::consts::PI/180.);
            )*
        }
    };
}

/// Sources:
/// https://gist.github.com/earthbound19/e7fe15fdf8ca3ef814750a61bc75b5ce
mod conversion {
    use crate::math::{Transform, Vec3, vec3};
    use crate::utils::{cos, sin};

    fn gamma_to_linear(c: f32) -> f32 {
        if c >= 0.04045 {((c + 0.055) / 1.055).powf(2.4)} else {c / 12.92}
    }
    fn srgb_to_rgb(rgb: Vec3) -> Vec3 {
        rgb.map_comp(gamma_to_linear)
    }
    fn rgb_to_lms(rgb: Vec3) -> Vec3 {
        const RGB_TO_LMS: Transform = Transform::from_array([
            0.4122214708, 0.2119034982, 0.0883024619,
            0.5363325363, 0.6806995451, 0.2817188376,
            0.0514459929, 0.1073969566, 0.6299787005,
            0., 0., 0.,
        ]);
        RGB_TO_LMS.tr_vec(rgb)
    }
    fn lms_to_oklab(lms: Vec3) -> Vec3 {
        let lms_ = lms.map_comp(f32::cbrt);
        const LMS__TO_OKLAB: Transform = Transform::from_array([
             0.2104542553,  1.9779984951,  0.0259040371,
             0.7936177850, -2.4285922050,  0.7827717662,
            -0.0040720468,  0.4505937099, -0.8086757660,
            0., 0., 0.
        ]);
        LMS__TO_OKLAB.tr_vec(lms_)
    }
    pub fn rgb_to_oklab(rgb: Vec3) -> Vec3 {
        lms_to_oklab(rgb_to_lms(rgb))
    }

    pub const fn oklch_to_oklab(oklch: Vec3) -> Vec3 {
        vec3(
            oklch.x(),
            cos(oklch.z()) * oklch.y(),
            sin(oklch.z()) * oklch.y(),
        )
    }
}

/// OKLAB color
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Color(Vec3);
impl Color {
    pub fn as_array(self) -> CompressedVec {
        compress_vec4_i(self.0.to_vec4(1.0))
    }
    pub fn from_rgbv(rgb: Vec3) -> Self {
        Self(conversion::rgb_to_oklab(rgb))
    }
    pub fn from_rgbf(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgbv(vec3(r, g, b))
    }
    pub const fn from_oklchv(oklch: Vec3) -> Self {
        Self(conversion::oklch_to_oklab(oklch))
    }
    pub const fn from_oklchf(l: f32, c: f32, h: f32) -> Self {
        Self::from_oklchv(vec3(l, c, h))
    }
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        Self(vec3(l, a, b))
    }
}

// codes found using https://oklch.com/
consts_lch!(
    WHITE = 1., 0., 0.;
    BLACK = 0., 0., 0.;
    DEBUG = 0.5, 0.2, 320.;
    RED = 0.5, 0.2, 30.;
    GREEN = 0.5, 0.2, 140.;
    BLUE = 0.5, 0.2, 267.;
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

#[test]
fn test() {
    dbg!(Color::WHITE, Color::RED, Color::BLUE);
}
