use crate::{materials::alloc::BuffersAllocPosition, world::entity::Entity};


#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityRef(usize);
impl EntityRef {
    pub const ROOT: Self = Self(0);
}

pub struct World {
    pub entities: Vec<Entity>,
    pub alloc: BuffersAllocPosition,
}
impl World {
    pub fn new() -> Self {
        Self {
            alloc: BuffersAllocPosition::new(),
            entities: Vec::new(),
        }
    }
    pub fn get(&self, eref: EntityRef) -> &Entity {
        &self.entities[eref.0]
    }
    pub fn get_mut(&mut self, eref: EntityRef) -> &mut Entity {
        &mut self.entities[eref.0]
    }
    pub fn set(&mut self, eref: EntityRef, ent: Entity) {
        *self.get_mut(eref) = ent;
    }
    pub fn add(&mut self, ent: Entity) -> EntityRef {
        self.entities.push(ent);
        EntityRef(self.entities.len()-1)
    }
}
