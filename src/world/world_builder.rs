use std::ops::Deref;
use std::{any::Any, collections::HashMap};

use crate::math::Transform;
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
    view_bounding_box: Option<Box<dyn Variator<Item = Transform>>>,
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
    layer: usize,
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
    pub fn set_bounding_box(&mut self, v: impl Variator<Item = Transform>) {
        self.state.view_bounding_box = Some(Box::new(v));
    }

    pub fn finalize(self) -> WorldsBuilder {
        let mut worlds = self.worlds;
        worlds.worlds[self.id.get()] = Some(self.state);
        while worlds.id_by_layers.len() <= self.layer {
            worlds.id_by_layers.push(Vec::new());
        }
        worlds.id_by_layers[self.layer].push(self.id);
        worlds
    }
}

pub struct WorldBuilderFinalizationValue {
    pub worlds: Vec<World>,
    pub id_by_layer: Vec<Vec<WorldId>>,
    pub camera_offsets: Vec<usize>,
    pub buffer_allocations: Vec<BufferAllocator>,
}

pub struct WorldsBuilder {
    worlds: Vec<Option<WorldBuildState>>,
    id_by_layers: Vec<Vec<WorldId>>,
}
impl Default for WorldsBuilder {
    fn default() -> Self {
        Self {
            worlds: Vec::new(),
            id_by_layers: Vec::new(),
        }
    }
}
impl WorldsBuilder {
    pub fn add_world(mut self, layer: usize) -> WorldBuilder {
        let id = WorldId(self.worlds.len());
        self.worlds.push(None);
        WorldBuilder {
            state: WorldBuildState::default(),
            worlds: self,
            id,
            layer,
        }
    }
    pub fn add_world_with(&mut self, layer: usize, f: impl FnOnce(&mut WorldBuilder)) {
        let worlds = std::mem::take(self); // Cheap
        let mut builder = worlds.add_world(layer);
        f(&mut builder);
        *self = builder.finalize();
    }
    pub fn finalize(mut self) -> WorldBuilderFinalizationValue {
        let w = self.worlds.last_mut().unwrap().as_mut().unwrap();
        let idx = Camera::alloc(&mut w.allocs_tracker, 1);
        w.variators.push(Box::new(SavedVariatorSingle {
            index: idx,
            var: GetManualCamera,
        }));

        let allocs = self
            .worlds
            .iter()
            .map(|wopt| match wopt {
                None => BufferAllocator::new(),
                Some(state) => state.allocs(),
            })
            .collect();

        let mut camera_offsets = vec![0; self.worlds.len() + 1];
        for i in 0..self.worlds.len() {
            camera_offsets[i + 1] =
                camera_offsets[i] + &self.worlds[i].as_ref().map(|w| w.allocs_tracker.camera).unwrap_or(0)
        }

        let worlds = self
            .worlds
            .into_iter()
            .map(|wopt| match wopt {
                None => World::new(),
                Some(state) => World {
                    directives: state.directives,
                    stores: state.allocs_tracker.to_store_holder(),
                    variators: state.variators,
                    view_bounding_box: state.view_bounding_box,
                    ..World::new()
                },
            })
            .collect();

        WorldBuilderFinalizationValue {
            worlds,
            camera_offsets,
            id_by_layer: self.id_by_layers,
            buffer_allocations: allocs,
        }
    }
}
