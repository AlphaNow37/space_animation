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
    pub referenced_worlds: Vec<usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            stores: PrimitiveStoresHolder::default(),
            directives: Vec::new(),
            settings: WorldSettings::default(),
            variators: Vec::new(),
            referenced_worlds: Vec::new(),
        }
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
            label.write(ctx.stores[label as usize], &mut self.stores);
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
