use std::cell::Cell;
use crate::render_registry::storage_structs::AsStrorageStruct;

use super::{variators::variator::{UpdateCtx, Variator}, world::World};

pub struct Register<T> {
    vars: Vec<(Cell<T>, Box<dyn Variator<Item=T>>)>,
}
impl<T: Copy + Default + 'static> Register<T> {
    pub fn new() -> Self {
        Self {
            vars: Vec::new()
        }
    }
    pub fn update(&self, idx: usize, ctx: UpdateCtx, world: &World) {
        let (cell, var) = &self.vars[idx];
        cell.set(var.update(ctx, world));
    }
    pub fn get(&self, idx: usize) -> T {
        self.vars[idx].0.get()
    }
    pub fn get_mod(&self, idx: isize) -> T {
        let idx = idx.rem_euclid(self.vars.len() as isize) as usize;
        self.get(idx)
    }
    pub fn push(&mut self, var: impl Variator<Item=T>+'static) -> usize {
        let idx = self.vars.len();
        self.vars.push((Cell::new(T::default()), Box::new(var)));
        idx
    }
    pub fn len(&self) -> usize {
        self.vars.len()
    }
    pub fn write(&self, store: &mut [u32]) where T: AsStrorageStruct {
        let arr: &mut [T::S] = bytemuck::cast_slice_mut(store);
        for i in 0..self.vars.len() {
            arr[i] = self.vars[i].0.get().as_strorage_struct();
        }
    }
}
