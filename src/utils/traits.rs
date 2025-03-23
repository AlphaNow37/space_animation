use crate::utils::make_trait_alias;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Zero {
    const ZERO: Self;
}
impl Zero for f32 {
    const ZERO: Self = 0.;
}
impl Zero for usize {
    const ZERO: Self = 0;
}
impl<T: Zero, const N: usize> Zero for [T; N] {
    const ZERO: Self = [T::ZERO; N];
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

pub trait Length: VectorSpace {
    fn length_squared(self) -> f32;
    fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }
    fn with_length(self, length: f32) -> Self {
        self * length / self.length()
    }
    fn with_length_squared(self, length: f32) -> Self {
        self * (length / self.length_squared()).sqrt()
    }
    fn with_length_or_zero_squared(self, length: f32) -> Self {
        let l = self.length_squared();
        if l == 0. {
            Self::ZERO
        } else {
            self * (length / l).sqrt()
        }
    }
    fn with_length_or_zero(self, length: f32) -> Self {
        let len = self.length();
        if len == 0. {
            Self::ZERO
        } else {
            self * length / len
        }
    }
    fn normalize(self) -> Self {
        self / self.length()
    }
    fn normalize_or_zero(self) -> Self {
        let len = self.length();
        if len == 0. { Self::ZERO } else { self / len }
    }
    fn is_normalized(self) -> bool {
        let l = self.length_squared();
        0.99 < l && l < 1.01
    }
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 0.0001
    }
}

// pub trait VectorSpace: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero {
//     fn mid(self, other: Self) -> Self {
//         (self + other) * 0.5
//     }
// }
// impl<T: Sized+Add<Output=Self> + Sub<Output=Self> + Mul<f32, Output=Self> + Div<f32, Output=Self> + Copy + Zero> VectorSpace for T {}

/// Can hash floats. Is just supposed to be a utility to find variables with the same value
pub trait GeneralHash {
    fn gen_hash(&self) -> u32;
}
macro_rules! impl_hash {
    (
        $self: ident,
        $(
            $ty: ty:
            $(<$($gen: ident), * $(,)?>)?
            $([$($gen2: tt)*])?
            { $expr: expr }
        );* $(;)?
    ) => {
        $(
            impl <$($($gen: GeneralHash, )*)? $($($gen2)*)?> GeneralHash for $ty {
                fn gen_hash(&$self) -> u32 {
                    $expr
                }
            }
        )*
    };
}

fn gen_hash_stdhash(v: &impl std::hash::Hash) -> u32 {
    struct Hasher(u32);
    impl std::hash::Hasher for Hasher {
        fn finish(&self) -> u64 {
            unimplemented!()
        }
        fn write(&mut self, bytes: &[u8]) {
            self.0 = (self.0 << 1) ^ bytes.gen_hash();
        }
        fn write_u32(&mut self, i: u32) {
            self.0 = (self.0 << 1) ^ i
        }
        fn write_u64(&mut self, i: u64) {
            self.0 = ((self.0 << 1) ^ (i as u32) << 1) ^ ((i >> 32) as u32)
        }
    }
    let mut h = Hasher(0);
    v.hash(&mut h);
    h.0
}

use crate::{
    math::{Angle, Dir, Mat4, Plane, Polynomial, Transform, Vec2, Vec3, Vec4},
    world::primitives::{camera::Camera, color::Color},
};
use std::any::TypeId;
impl_hash!(
    self,
    f32: {self.to_bits().into()};
    usize: {*self as u32};
    u8: {*self as u32};
    u32: {*self};
    (A, B): <A, B> {(self.0.gen_hash() << 1) ^ self.1.gen_hash()};
    (A, B, C): <A, B, C> {((self.0.gen_hash()<<1) ^ self.1.gen_hash() << 1) ^ self.2.gen_hash()};
    (A, B, C, D): <A, B, C, D> {(((self.0.gen_hash()<<1) ^ self.1.gen_hash()<<1) ^ self.2.gen_hash()<<1) ^ self.3.gen_hash()};
    &[T]: <T> {self.iter().map(|e| e.gen_hash()).fold(0, |a, b| (a<<1)^b)};
    [T; N]: <T> [const N: usize] {self.iter().map(|e| e.gen_hash()).fold(0, |a, b| (a<<1)^b)};
    Angle: {self.rad().rem_euclid(std::f32::consts::TAU).gen_hash()};
    Vec2: {self.0.as_array().gen_hash()};
    Vec3: {(&self.0.as_array()[..3]).gen_hash()};
    Vec4: {self.0.as_array().gen_hash()};
    Mat4: {self.0.as_array().gen_hash()};
    Transform: {self.0.as_array().gen_hash()};
    Polynomial<T, N, M>: <T> [const N: usize, const M: usize] {self.0.gen_hash()};
    Dir: {(**self).gen_hash()};
    Plane: {self.normal().gen_hash()};
    Color: {self.to_array().gen_hash()};
    Camera: {(self.fov, self.pos).gen_hash()};
    TypeId: {gen_hash_stdhash(self)};
);
