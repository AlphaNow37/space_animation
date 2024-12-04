use crate::world::world::World;


mod fightsky;
mod tests;

pub fn build(world: &mut World) {
    fightsky::build(world);
}
