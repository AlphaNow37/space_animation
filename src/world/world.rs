use crate::{materials::alloc::BuffersAllocPosition, world::entity::Entity};

use super::view::EntityView;


#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub struct EntityRef(usize);
impl EntityRef {
    pub const ROOT: Self = Self(0);
    pub fn as_usize(&self) -> usize {self.0}
}

pub struct World {
    pub entities: Vec<Entity>,
}
impl World {
    pub fn new() -> Self {
        Self {
            entities: vec![Entity::default()],
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
    pub fn view(&mut self, eref: EntityRef) -> EntityView {
        EntityView {world: self, eref}
    }
}
