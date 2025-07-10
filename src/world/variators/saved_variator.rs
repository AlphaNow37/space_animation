use std::any::Any;

use crate::world::primitives::WorldPrimitive;
use crate::world::world::Worlds;

use super::variator::Variator;

pub trait SavedVariator: Any {
    fn write(&self, worlds: &Worlds);
}
pub struct SavedVariatorSingle<V> {
    pub var: V,
    pub index: usize,
}
impl<V: Variator> SavedVariator for SavedVariatorSingle<V>
where
    V::Item: WorldPrimitive,
{
    fn write(&self, worlds: &Worlds) {
        let res = self.var.update(worlds);
        V::Item::set(&worlds.world.stores, self.index, res);
    }
}
pub struct SavedVariatorMultiple<V> {
    pub var: V,
    pub index: usize,
}
impl<const N: usize, T, V: Variator<Item = [T; N]>> SavedVariator for SavedVariatorMultiple<V>
where
    T: WorldPrimitive,
{
    fn write(&self, worlds: &Worlds) {
        let res = self.var.update(worlds);
        T::sets(&worlds.world.stores, self.index, res);
    }
}
