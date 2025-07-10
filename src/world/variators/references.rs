use std::marker::PhantomData;

use crate::world::{primitives::WorldPrimitive, world_builder::WorldId};
use crate::world::world::Worlds;

use super::variator::Variator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ref<T> {
    label: PhantomData<fn() -> T>,
    index: usize,
    world_id: WorldId,
}
impl<T> Ref<T> {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn world_id(&self) -> WorldId {
        self.world_id
    }
}
impl<T: WorldPrimitive> Variator for Ref<T> {
    type Item = T;
    fn update(&self, worlds: &Worlds) -> Self::Item {
        T::get(&worlds.worlds[self.world_id.get()].stores, self.index)
    }
    fn hash_var(&self) -> u32 {
        self.index as u32
    }
    fn eq_var(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

// Should only be used by world builders
pub fn make_raw_ref<T>(index: usize, world_id: WorldId) -> Ref<T> {
    Ref {
        index,
        world_id,
        label: PhantomData,
    }
}
