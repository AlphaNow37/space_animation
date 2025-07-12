use crate::math::{Transform, Vec3};
use crate::utils::Zero;
use crate::world::visuals::Triangle;
use crate::world::world_builder::WorldBuilder;
use crate::world::{primitives::color::Color, variators::variator::Variator};

pub fn put_axis(world: &mut WorldBuilder, pos: impl Variator<Item = Transform> + Copy) {
    let global = world.push(pos);
    world.push_visual(global);

    let o = world.push(Vec3::ZERO);
    let x = world.push(Vec3::X);
    let y = world.push(Vec3::Y);
    let z = world.push(Vec3::Z);

    for (a, b, col) in [
        (x, y, Color::GREEN),
        (y, z, Color::RED),
        (z, x, Color::BLUE),
    ] {
        let col = world.push(col);
        world.push_visual((col, Triangle(o, a, b)))
    }
}
