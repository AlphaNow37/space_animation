
use glam::{Affine3A, Vec3A};

use crate::world::{color::Color, material::UniformTri, point::{ProjectPoint, Translation}, shape::Triangle, variator::Variator, world::World};

pub fn put_axis(world: &mut World, pos: impl Variator<Item=Affine3A>+Copy) {
    let or = Translation(pos);
    let x = ProjectPoint(pos, Vec3A::X);
    let y = ProjectPoint(pos, Vec3A::Y);
    let z = ProjectPoint(pos, Vec3A::Z);
    world.push_mat(
        UniformTri {
            shape: Triangle(or, x, y),
            color: Color::GREEN,
        }
    );
    world.push_mat(
        UniformTri {
            shape: Triangle(or, x, z),
            color: Color::BLUE,
        }
    );
    world.push_mat(
        UniformTri {
            shape: Triangle(or, y, z),
            color: Color::RED,
        }
    );
}

