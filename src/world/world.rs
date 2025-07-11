use crate::math::Transform;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::render_registry::vertex::VertexType;
use crate::world::primitives::camera::Camera;
use crate::world::variators::variator::Variator;
use crate::world::visuals::VisualDirective;

use super::primitives::{PrimitiveStoresHolder, StoreLabel, WorldPrimitive};
use super::variators::saved_variator::SavedVariator;

pub struct World {
    pub stores: PrimitiveStoresHolder,
    pub variators: Vec<Box<dyn SavedVariator>>,
    pub settings: WorldSettings,
    pub directives: Vec<Box<dyn VisualDirective>>,
    pub view_bounding_box: Option<Box<dyn Variator<Item = Transform>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            stores: PrimitiveStoresHolder::default(),
            directives: Vec::new(),
            settings: WorldSettings::default(),
            variators: Vec::new(),
            view_bounding_box: None,
        }
    }

    pub fn redraw(&self, instance_bufs: [[&mut [u32]; MaterialType::COUNT]; VertexType::COUNT]) {
        let mut executor = VisualExecutor::new(instance_bufs);
        for dir in &self.directives {
            dir.exec(&mut executor)
        }
    }
    pub fn update_registers(&self, worlds: &Worlds) {
        for saved_var in &self.variators {
            saved_var.write(worlds);
        }
    }
    pub fn write_stores(&self, stores: [&mut [u32]; StoreLabel::COUNT]) {
        for label in StoreLabel::ARRAY {
            label.write(stores[label as usize], &self.stores);
        }
    }
    pub fn get_cam(&self, idx: usize) -> Camera {
        Camera::get(&self.stores, idx)
    }
}

#[derive(Default)]
pub struct WorldSettings {
    pub cam_settings: Camera,
    pub base_time: f32,
}

pub struct Worlds<'a> {
    pub world: &'a World,
    pub worlds: &'a [World],
    pub settings: WorldSettings,
}
