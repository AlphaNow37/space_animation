use crate::math::{Vec3, Transform};
use crate::utils::Zero;
use crate::world::{primitives::color::Color, variators::variator::Variator, world::World};
use crate::world::point::ProjectPoint;
use crate::world::variators::variator::UpdateCtx;
use crate::world::visuals::material::UniformTri;
use crate::world::visuals::shape::Triangle;

pub fn put_axis(world: &mut World, pos: impl Variator<Item=Transform>+Copy) {
    let global = world.push_stored(pos);
    let o = world.push_stored(Vec3::ZERO);
    let x = world.push_stored(Vec3::X);
    let y = world.push_stored(Vec3::Y);
    let z = world.push_stored(Vec3::Z);
    let red = world.push_stored(Color::RED);
    let green = world.push_stored(Color::GREEN);
    let blue = world.push_stored(Color::BLUE);

    world.push_mat(UniformTri {
        shape: Triangle(o, x, y),
        global,
        color: green,
    });
    world.push_mat(UniformTri {
        shape: Triangle(o, y, z),
        global,
        color: red,
    });
    world.push_mat(UniformTri {
        shape: Triangle(o, z, x),
        global,
        color: blue,
    });
    //
    // for (a, b, col) in [
    //     (Vec3::X, Vec3::Y, Color::GREEN),
    //     (Vec3::X, Vec3::Z, Color::BLUE),
    //     (Vec3::Y, Vec3::Z, Color::RED),
    // ] {
    //     world.push_mat(
    //         UniformTri {
    //             shape: Triangle(Vec3::ZERO, a, b),
    //             color: col,
    //             global: pos,
    //         }
    //     );
    // }
}
