use crate::render_registry::materials::MaterialType;
use crate::render_registry::mesh_builder::VisualExecutor;
use crate::render_registry::vertex::VertexType;
use crate::world::primitives::camera::Camera;
use crate::world::visuals::VisualDirective;

use super::primitives::{PrimitiveStoresHolder, StoreLabel, WorldPrimitive};
use super::variators::saved_variator::SavedVariator;

pub struct World {
    pub stores: PrimitiveStoresHolder,
    pub variators: Vec<Box<dyn SavedVariator>>,
    pub settings: WorldSettings,
    pub directives: Vec<Box<dyn VisualDirective>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            stores: PrimitiveStoresHolder::default(),
            directives: Vec::new(),
            settings: WorldSettings::default(),
            variators: Vec::new(),
        }
    }

    fn redraw(&self, ctx: &mut WorldUpdateCtx) {
        let mut executor = VisualExecutor::new(std::mem::take(&mut ctx.instance_bufs));
        for dir in &self.directives {
            dir.exec(&mut executor)
        }
    }
    fn update_registers(&self, worlds: &Worlds) {
        for saved_var in &self.variators {
            saved_var.write(worlds);
        }
    }
    pub fn update(&self, mut ctx: WorldUpdateCtx) {
        self.update_registers(ctx.worlds);
        for label in StoreLabel::ARRAY {
            label.write(ctx.stores[label as usize], &self.stores);
        }
        self.redraw(&mut ctx);
    }
    pub fn get_cam(&self, idx: isize) -> Camera {
        Camera::get(
            &self.stores,
            idx.rem_euclid(self.stores.nb_cameras() as isize) as usize,
        )
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

pub struct WorldUpdateCtx<'a> {
    pub instance_bufs: [[&'a mut [u32]; MaterialType::COUNT]; VertexType::COUNT],
    pub stores: [&'a mut [u32]; StoreLabel::COUNT],
    pub worlds: &'a Worlds<'a>,
}
