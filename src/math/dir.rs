use crate::math::{Angle, Plane, Vec3, rotate_x};
use crate::utils::{Length, Zero};
use std::ops::{Deref, Neg};

/// A normalized vector
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dir(Vec3);
impl Dir {
    pub const X: Self = Self(Vec3::X);
    pub const Y: Self = Self(Vec3::Y);
    pub const Z: Self = Self(Vec3::Z);
    pub fn from_normalized(vec: Vec3) -> Self {
        Self(vec)
    }
    pub fn project(self, vec: Vec3) -> Vec3 {
        self.0 * vec.dot(self.0)
    }
    pub fn any_ortho(self) -> Self {
        match Plane::from_normal(Dir::Z).project(self.0).dir() {
            None => Dir::X,
            Some(v) => Self(rotate_x(Angle::from_deg(90.)).tr_vec(v.0)),
        }
    }
}
impl TryFrom<Option<Dir>> for Dir {
    type Error = ();
    fn try_from(value: Option<Dir>) -> Result<Self, Self::Error> {
        value.ok_or(())
    }
}
impl TryFrom<Vec3> for Dir {
    type Error = ();
    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        (value != Vec3::ZERO)
            .then(|| Self(value.normalize()))
            .ok_or(())
    }
}
impl Deref for Dir {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Default for Dir {
    fn default() -> Self {
        Self::Z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Vec3::ONE.length(), 3.0f32.sqrt());
        // dbg!(Vec3::ONE.rotate_around(Vec3::new(1., 1., 0.), Angle::from_deg(60.)));
    }
}
