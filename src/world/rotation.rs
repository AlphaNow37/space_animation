use std::ops::{Add, Div, Mul, Neg, Sub};


/// A radian angle
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Angle(f32);
impl Angle {
    pub fn from_rad(rad: f32) -> Self {Self(rad)}
    pub fn from_deg(deg: f32) -> Self {Self(deg.to_radians())}
    pub fn rad(self) -> f32 {self.0}
    pub fn deg(self) -> f32 {self.0.to_degrees()}
}
// TODO: replace with derive from (derive_more?)
impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<f32> for Angle {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Div<f32> for Angle {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl Neg for Angle {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
