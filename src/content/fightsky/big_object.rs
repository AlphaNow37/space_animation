use rand::Rng;

use crate::{math::Vec3, world::{variators::variator::Variator, world::World}};


fn place_big_object(world: &mut World, position: impl Variator<Item=Vec3>, rng: &mut impl Rng) {

}

pub fn place_big_objects(world: &mut World, rng: &mut impl Rng) {
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                place_big_object(world, Vec3::new(x as f32, y as f32, z as f32) * 100., rng);
            }
        }
    }
}
