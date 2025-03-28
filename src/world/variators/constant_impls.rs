use crate::math::Angle;
use crate::math::{Dir, Polynomial, Transform, Vec2, Vec3, Vec4};
use crate::utils::traits::GeneralHash;
use crate::world::primitives::camera::Camera;

use crate::world::primitives::color::Color;
use crate::world::variators::variator::Variator;
use crate::world::world::World;

macro_rules! make_constant_variators {
    (
        $($name: ty);* $(;)?
    ) => (
        $(
            impl Variator for $name {
                type Item=$name;
                fn hash_var(&self) -> u32 {
                    self.gen_hash()
                }
                fn eq_var(&self, other: &Self) -> bool {
                    self == other
                }
                fn update(&self, _world: &World) -> $name {
                    *self
                }
            }
        )*
    )
}

make_constant_variators!(
    Vec3;
    Transform;
    Color;
    f32;
    Vec2;
    Camera;
    Angle;
    Dir;
    Vec4;
    (Color, Color);
    Polynomial<Vec3, 4, 4>;
);
