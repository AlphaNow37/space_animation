use crate::world::world::{EntityRef, World};

pub struct EntityView<'a> {
    world: &'a mut World,
    eref: EntityRef,
}
