use std::any::Any;
use crate::world::variators::variator::Variator;
use crate::world::world::{World, WorldPrimitive};

pub trait SavedVariator: Any {
    fn write(&self, world: &World);
}
pub struct SavedVariatorSingle<V> {
    pub var: V,
    pub index: usize,
}
impl<V: Variator> SavedVariator for SavedVariatorSingle<V> where V::Item: WorldPrimitive {
    fn write(&self, world: &World) {
        let res = self.var.update(world);
        V::Item::set(world, self.index, res);
    }
}
pub struct SavedVariatorMultiple<V> {
    pub var: V,
    pub index: usize,
}
impl<const N: usize, T, V: Variator<crate::world::variators::variator::Item=[T; N]>> SavedVariator for SavedVariatorMultiple<V> where T: WorldPrimitive {
    fn write(&self, world: &World) {
        let res = self.var.update(world);
        T::sets(world, self.index, res);
    }
}
