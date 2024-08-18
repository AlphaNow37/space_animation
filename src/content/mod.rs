use glam::Vec3A;
use crate::world::{color::Color, combinators::Interpolate, material::UniformTri, pack::Pack3, shape::Triangle, variator::Variator, world::World};

pub fn build(world: &mut World) {
    let a = world.push(Interpolate(0., 1.).time_mod(1.).time_mul(0.2));
    world.push_mat(
        UniformTri {
            shape: Triangle(Vec3A::new(0., 0., 0.), Vec3A::new(0., 1., 0.), Vec3A::new(1., 0., 0.)),
            color: Color::RED,
        }
    );
    world.push_mat(
        UniformTri {
            shape: Triangle(Vec3A::new(0., 0., 0.), Vec3A::new(0., 0., 1.), Vec3A::new(1., 0., 0.)),
            color: Color::GREEN,
        }
    );
    world.push_mat(
        UniformTri {
            shape: Triangle(Vec3A::new(0., 0., 0.), Vec3A::new(0., 0., 1.), Vec3A::new(0., 1., 0.)),
            color: Color::BLUE,
        }
    );
}
