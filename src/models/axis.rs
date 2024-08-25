use crate::math::{Vec3, Transform};
use crate::utils::Zero;
use crate::world::{primitives::color::Color, variators::variator::Variator, world::World};
use crate::world::point::ProjectPoint;
use crate::world::variators::variator::UpdateCtx;
use crate::world::visuals::{SetGlobal, Uniform};
use crate::world::visuals::shape::Triangle;

pub fn put_axis(world: &mut World, pos: impl Variator<Item=Transform>+Copy) {
    let global = world.push(pos);
    let o = world.push(Vec3::ZERO);
    let x = world.push(Vec3::X);
    let y = world.push(Vec3::Y);
    let z = world.push(Vec3::Z);
    let red = world.push(Color::RED);
    let green = world.push(Color::GREEN);
    let blue = world.push(Color::BLUE);

    world.push_visual(SetGlobal(global));
    world.push_visual(Uniform(green));
    world.push_visual(Triangle(o, x, y));
    world.push_visual(Uniform(red));
    world.push_visual(Triangle(o, y, z));
    world.push_visual(Uniform(blue));
    world.push_visual(Triangle(o, z, x));
}
