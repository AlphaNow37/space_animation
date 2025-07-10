use std::collections::HashSet;
use std::ops::Deref;
use std::{any::Any, collections::HashMap};

use rand::seq::IndexedRandom;

use crate::render_registry::alloc::BufferAllocator;
use crate::render_registry::materials::MaterialType;

use super::primitives::camera::{Camera, GetManualCamera};
use super::world::World;
use super::{
    primitives::{PrimitivesAllocationTracker, WorldPrimitive},
    variators::{
        references::Ref,
        saved_variator::{SavedVariator, SavedVariatorMultiple, SavedVariatorSingle},
        variator::Variator,
    },
    visuals::VisualDirective,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WorldId(usize);
impl WorldId {
    pub fn get(self) -> usize {
        self.0
    }
}

#[derive(Default)]
struct WorldBuildState {
    variators_cache: HashMap<u32, usize>,
    directives: Vec<Box<dyn VisualDirective>>,
    variators: Vec<Box<dyn SavedVariator>>,
    pub allocs_tracker: PrimitivesAllocationTracker,
    parent_worlds: HashSet<WorldId>,
}
impl WorldBuildState {
    pub fn allocs(&self) -> BufferAllocator {
        let mut alloc = BufferAllocator::new();
        let mut curr_mty = MaterialType::default();
        for vis in &self.directives {
            vis.alloc(&mut curr_mty, &mut alloc);
        }
        self.allocs_tracker.allocs(&mut alloc);
        alloc
    }
}

pub struct WorldBuilder {
    state: WorldBuildState,
    worlds: WorldsBuilder,
    id: WorldId,
}

impl WorldBuilder {
    fn make_ref<T>(&self, idx: usize) -> Ref<T> {
        crate::world::variators::references::make_raw_ref(idx, self.id)
    }
    pub fn push_visual(&mut self, vis: impl VisualDirective + 'static) {
        self.state.directives.push(Box::new(vis));
    }
    pub fn push<V: Variator>(&mut self, var: V) -> Ref<V::Item>
    where
        V::Item: WorldPrimitive,
    {
        let hash = var.finished_hash_var();
        let mut add = false;
        if let Some(&var_idx) = self.state.variators_cache.get(&hash) {
            let v: &dyn SavedVariator = Box::deref(&self.state.variators[var_idx]);
            let v: &dyn Any = v;
            if let Some(SavedVariatorSingle { index, var: var2 }) =
                v.downcast_ref::<SavedVariatorSingle<V>>()
            {
                if var.eq_var(var2) {
                    return self.make_ref(*index);
                }
            }
        } else {
            add = true;
        }
        if add {
            self.state
                .variators_cache
                .insert(hash, self.state.variators.len());
        }
        let idx = V::Item::alloc(&mut self.state.allocs_tracker, 1);
        self.state
            .variators
            .push(Box::new(SavedVariatorSingle { index: idx, var }));
        self.make_ref(idx)
    }
    pub fn push_multi<const N: usize, T: WorldPrimitive, V: Variator<Item = [T; N]>>(
        &mut self,
        var: V,
    ) -> [Ref<T>; N] {
        if N == 0 {
            return std::array::from_fn(|_| unreachable!());
        }
        let hash = var.finished_hash_var();
        let mut add = false;
        if let Some(&var_idx) = self.state.variators_cache.get(&hash) {
            let v: &dyn SavedVariator = Box::deref(&self.state.variators[var_idx]);
            let v: &dyn Any = v;
            if let Some(SavedVariatorMultiple { index, var: var2 }) =
                v.downcast_ref::<SavedVariatorMultiple<V>>()
            {
                if var.eq_var(var2) {
                    return std::array::from_fn(|i| self.make_ref(index + i));
                }
            }
        } else {
            add = true;
        }
        if add {
            self.state
                .variators_cache
                .insert(hash, self.state.variators.len());
        }
        let idx = T::alloc(&mut self.state.allocs_tracker, N);
        self.state
            .variators
            .push(Box::new(SavedVariatorMultiple { index: idx, var }));
        std::array::from_fn(|i| self.make_ref(idx + i))
    }
    pub fn using_ref<T>(&mut self, rf: Ref<T>) {
        self.state.parent_worlds.insert(rf.world_id());
    }

    pub fn finalize(self) -> WorldsBuilder {
        let mut worlds = self.worlds;
        worlds.worlds[self.id.get()] = Some(self.state);
        worlds
    }
}

pub struct WorldsBuilder {
    worlds: Vec<Option<WorldBuildState>>,
}
impl Default for WorldsBuilder {
    fn default() -> Self {
        Self { worlds: Vec::new() }
    }
}
impl WorldsBuilder {
    pub fn add_world(mut self) -> WorldBuilder {
        let id = WorldId(self.worlds.len());
        self.worlds.push(None);
        WorldBuilder {
            state: WorldBuildState::default(),
            worlds: self,
            id,
        }
    }
    pub fn add_world_with(&mut self, f: impl FnOnce(&mut WorldBuilder)) {
        let worlds = std::mem::take(self); // Cheap
        let mut builder = worlds.add_world();
        f(&mut builder);
        *self = builder.finalize();
    }
    pub fn finalize(&mut self) {
        let w = self.worlds.last_mut().unwrap().as_mut().unwrap();
        let idx = Camera::alloc(&mut w.allocs_tracker, 1);
        w.variators.push(Box::new(SavedVariatorSingle {
            index: idx,
            var: GetManualCamera,
        }));
    }
    pub fn get_buffer_allocs(&self) -> Vec<BufferAllocator> {
        self.worlds
            .iter()
            .map(|wopt| match wopt {
                None => BufferAllocator::new(),
                Some(state) => state.allocs(),
            })
            .collect()
    }
    pub fn to_run_worlds(self) -> Vec<World> {
        let mut child_worlds = vec![vec![]; self.worlds.len()];
        for (i, s) in self.worlds.iter().enumerate() {
            if let Some(w) = s {
                for id in &w.parent_worlds {
                    child_worlds[id.get()].push(i);
                }
            }
        }
        let mut run_worlds: Vec<World> = self
            .worlds
            .into_iter()
            .map(|wopt| match wopt {
                None => World::new(),
                Some(state) => World {
                    directives: state.directives,
                    stores: state.allocs_tracker.to_store_holder(),
                    variators: state.variators,
                    parent_worlds: state.parent_worlds.iter().map(|id| id.get()).collect(),
                    child_worlds: vec![],
                    ..World::new()
                },
            })
            .collect();
        for (i, chs) in child_worlds.into_iter().enumerate() {
            run_worlds[i].child_worlds = chs;
        }
        run_worlds
    }
}
