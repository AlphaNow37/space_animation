use crate::math::{Vec3, Transform};
use crate::world::{primitives::color::Color, variators::variator::Variator, world::World};
use crate::world::visuals::material::UniformTri;
use crate::world::visuals::shape::Triangle;

pub fn put_axis(world: &mut World, pos: impl Variator<Item=Transform>+Copy) {
    for (a, b, col) in [
        (Vec3::X, Vec3::Y, Color::GREEN),
        (Vec3::X, Vec3::Z, Color::BLUE),
        (Vec3::Y, Vec3::Z, Color::RED),
    ] {
        world.push_mat(
            UniformTri {
                shape: Triangle(Vec3::ZERO, a, b),
                color: col,
                global: pos,
            }
        );
    }
}
