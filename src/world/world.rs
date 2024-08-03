use crate::world::entity::{Entity};


#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityRef(usize);

pub struct World {
    entities: Vec<Entity>,
}
impl World {
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
