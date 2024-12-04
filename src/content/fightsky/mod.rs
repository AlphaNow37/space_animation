use rand::thread_rng;

use crate::world::world::World;

mod big_object;

pub fn build(world: &mut World) {
    let mut rng = thread_rng();
    big_object::place_big_objects(world, &mut rng);
}
