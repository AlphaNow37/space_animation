use rand::thread_rng;

use crate::{math::Transform, world::{primitives::color::Color, visuals::Sphere, world::World}};


pub fn build(world: &mut World) {
    let mut rng = thread_rng();

    let transform = world.push(Transform::ID);
    let col = world.push(Color::RED);
    world.push_visual((
        transform,
        col,
        Sphere(transform),
    ));
}
