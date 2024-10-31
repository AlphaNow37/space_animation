use std::marker::PhantomData;
use std::collections::HashMap;
use std::any::Any;
use std::ops::Deref;

use crate::render_registry::alloc::BufferAllocator;
use crate::world::primitives::camera::Camera;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::render_registry::vertex::VertexType;
use crate::world::stores::StoreLabel;
use crate::world::visuals::VisualDirective;
use crate::world::primitives::primitives::PrimitiveRegisters;
use crate::world::variators::saved_variator::{SavedVariator, SavedVariatorMultiple, SavedVariatorSingle};

use super::variators::variator::Variator;

#[derive(Default)]
pub struct World {
    registers: PrimitiveRegisters,
    variators: Vec<Box<dyn SavedVariator>>,
    variators_cache: HashMap<u32, usize>,
    pub settings: WorldSettings,
    directives: Vec<Box<dyn VisualDirective>>,
    sub_worlds: Vec<World>,
}

impl World {
    pub fn new() {

    }
    pub fn allocs(&self, alloc: &mut BufferAllocator) {
        let mut curr_mty = MaterialType::default();
        for vis in &self.directives {
            vis.alloc(&mut curr_mty, alloc);
        }
        for label in StoreLabel::ARRAY {
            label.alloc(self, alloc);
        }
    }
    pub fn push_visual(&mut self, vis: impl VisualDirective + 'static) {
        self.directives.push(Box::new(vis));
    }
    fn redraw(&self, ctx: &mut WorldUpdateCtx) {
        let mut executor = VisualExecutor::new(std::mem::take(&mut ctx.instance_bufs));
        for dir in &self.directives {
            dir.exec(&mut executor)
        }
    }
    fn update_settings(&mut self, ctx: &WorldUpdateCtx) {
        self.settings = WorldSettings {
            cam_settings: ctx.cam,
            base_time: ctx.time,
        }
    }
    fn update_registers(&self) {
        for saved_var in &self.variators {
            saved_var.write(self);
        }
    }
    pub fn update(&mut self, mut ctx: WorldUpdateCtx) {
        self.update_settings(&ctx);
        self.update_registers();
        for label in StoreLabel::ARRAY {
            label.write(ctx.stores[label as usize], self);
        }
        self.redraw(&mut ctx);
    }
    pub fn get_cam(&self, idx: isize) -> Camera {
        Camera::get(self, idx.rem_euclid(self.camera.len() as isize) as usize)
    }
    pub fn push<V: Variator>(&mut self, var: V) -> Ref<V::Item> where V::Item: WorldPrimitive {
        let hash = var.finished_hash_var();
        let mut add = false;
        if let Some(&var_idx) = self.variators_cache.get(&hash) {
            let v: &dyn SavedVariator = Box::deref(&self.variators[var_idx]);
            let v: &dyn Any = v;
            if let Some(SavedVariatorSingle { index, var: var2 }) = v.downcast_ref::<SavedVariatorSingle<V>>() {
                if var.eq_var(var2) {
                    return Ref {
                        index: *index,
                        label: PhantomData,
                    };
                }
            }
        } else {
            add = true;
        }
        if add {
            self.variators_cache.insert(hash, self.variators.len());
        }
        let idx = V::Item::alloc(self, 1);
        self.variators.push(Box::new(SavedVariatorSingle {
            index: idx,
            var,
        }));
        Ref {
            index: idx,
            label: PhantomData,
        }
    }
    pub fn push_multi<const N: usize, T: WorldPrimitive, V: Variator<Item=[T; N]>>(&mut self, var: V) -> [Ref<T>; N] {
        if N==0 {return std::array::from_fn(|_| Ref {index: 0, label: PhantomData})}
        let hash = var.finished_hash_var();
        let mut add = false;
        if let Some(&var_idx) = self.variators_cache.get(&hash) {
            let v: &dyn SavedVariator = Box::deref(&self.variators[var_idx]);
            let v: &dyn Any = v;
            if let Some(SavedVariatorMultiple { index, var: var2 }) = v.downcast_ref::<SavedVariatorMultiple<V>>() {
                if var.eq_var(var2) {
                    return std::array::from_fn(|i| Ref {
                        index: *index+i,
                        label: PhantomData,
                    });
                }
            }
        } else {
            add = true;
        }
        if add {
            self.variators_cache.insert(hash, self.variators.len());
        }
        let idx = T::alloc(self, N);
        self.variators.push(Box::new(SavedVariatorMultiple {
            index: idx,
            var,
        }));
        std::array::from_fn(|i| Ref {
            index: idx+i,
            label: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ref<T> {
    label: PhantomData<fn()->T>,
    index: usize,
}
impl<T> Ref<T> {
    pub fn index(&self) -> usize {self.index}
}
impl<T: WorldPrimitive> Variator for Ref<T> {
    type Item=T;
    fn update(&self, world: &World) -> T {
        T::get(world, self.index)
    }
    fn hash_var(&self) -> u32 {
        self.index as u32
    }
    fn eq_var(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub struct WorldUpdateCtx<'a> {
    pub instance_bufs: [[&'a mut [u32]; MaterialType::COUNT]; VertexType::COUNT],
    pub stores: [&'a mut [u32]; StoreLabel::COUNT],
    pub cam: Camera,
    pub time: f32,
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
    pub base_time: f32,
}

pub trait WorldPrimitive: Sized + 'static {
    fn alloc(world: &mut World, size: usize) -> usize;
    fn get(world: &World, index: usize) -> Self;
    fn set(world: &World, index: usize, value: Self);
    fn sets<const N: usize>(world: &World, index: usize, values: [Self; N]);
}
