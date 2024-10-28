use std::marker::PhantomData;
use std::cell::Cell;
use std::collections::HashMap;
use std::any::Any;
use std::ops::Deref;

use crate::render_registry::storage_structs::AsStrorageStruct;
use crate::math::{Vec2, Vec3, Vec4, Transform, Dir, Polynomial};
use crate::render_registry::alloc::BufferAllocator;
use crate::world::primitives::camera::Camera;
use crate::math::Angle;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::render_registry::vertex::VertexType;
use crate::world::stores::StoreLabel;
use crate::world::visuals::VisualDirective;
use crate::utils::traits::GeneralHash;

use super::variators::variator::Variator;
use super::primitives::color::Color;

macro_rules! make_system {
    (
        $(
            $attr: ident : $prim_ty: ident $(($store_method: ident, $len_method: ident))?
        );*
        $(;)?
    ) => {
            $(
                impl Variator for $prim_ty {
                    type Item=$prim_ty;
                    fn hash_var(&self) -> u32 {
                        self.gen_hash()
                    }
                    fn eq_var(&self, other: &Self) -> bool {
                        self == other
                    }
                    fn update(&self, _world: &World) -> $prim_ty {
                        *self
                    }
                }
                impl WorldPrimitive for $prim_ty {
                    fn alloc(world: &mut World, size: usize) -> usize {
                        let idx = world.$attr.len();
                        world.$attr.reserve(size);
                        for _ in 0..size {
                            world.$attr.push(Cell::new(Self::default()));
                        }
                        idx
                    }
                    fn get(world: &World, index: usize) -> Self {
                        world.$attr[index].get()
                    }
                    fn set(world: &World, index: usize, value: Self) {
                        world.$attr[index].set(value);
                    }
                    fn sets<const N: usize>(world: &World, index: usize, values: [Self; N]) {
                        for i in 0..values.len() {
                            Self::set(world, index+i, values[i]) // i hope this gets optimised away
                        }
                    }
                }
            )*

        pub struct World {
            $(
                $attr: Vec<Cell<$prim_ty>>,
            )*
            variators: Vec<Box<dyn SavedVariator>>,
            variators_cache: HashMap<u32, usize>,
            pub settings: WorldSettings,
            directives: Vec<Box<dyn VisualDirective>>,
        }
        impl World {
            pub fn new() -> Self {
                Self {
                    $(
                        $attr: Vec::new(),
                    )*
                    directives: Vec::new(),
                    settings: WorldSettings::default(),
                    variators: Vec::new(),
                    variators_cache: HashMap::new(),
                }
            }

            $($(
                pub fn $store_method(&self, buf: &mut [u32]) {
                    let arr: &mut [<$prim_ty as AsStrorageStruct>::S] = bytemuck::cast_slice_mut(buf);
                    for i in 0..self.$attr.len() {
                        arr[i] = self.$attr[i].get().as_strorage_struct();
                    }
                }
                pub fn $len_method(&self) -> usize {
                    self.$attr.len()
                }
            )?)*
        }
    };
}

type F32 = f32;
type Color2 = (Color, Color);
type Polynomial4x4 = Polynomial<Vec3, 4, 4>;
make_system!(
    vec3: Vec3 (store_vec3, len_vec3);
    transform: Transform (store_transform, len_transform);
    color: Color (store_color, len_color);
    f32: F32 (store_f32, len_f32);
    vec2: Vec2 (store_vec2, len_vec2);
    camera: Camera;
    angle: Angle;
    dir: Dir;
    vec4: Vec4 (store_vec4, len_vec4);
    color2: Color2 (store_color2, len_color2);
    polynomial4x4: Polynomial4x4 (store_poly4x4, len_poly4x4);
);

impl World {
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

trait SavedVariator: Any {
    fn write(&self, world: &World);
}
struct SavedVariatorSingle<V> {
    var: V,
    index: usize,
}
impl<V: Variator> SavedVariator for SavedVariatorSingle<V> where V::Item: WorldPrimitive {
    fn write(&self, world: &World) {
        let res = self.var.update(world);
        V::Item::set(world, self.index, res);
    }
}
struct SavedVariatorMultiple<V> {
    var: V,
    index: usize,
}
impl<const N: usize, T, V: Variator<Item=[T; N]>> SavedVariator for SavedVariatorMultiple<V> where T: WorldPrimitive {
    fn write(&self, world: &World) {
        let res = self.var.update(world);
        T::sets(world, self.index, res);
    }
}
