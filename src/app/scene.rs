use std::ops::DerefMut;

use log::info;
use tracing::info_span;
// use crate::app::App;
use crate::{render_registry::{alloc::{BuffersAllocPosition, Position}, registry::PipelinesRegistry}, world::{variator::UpdateCtx, world::{World, WorldUpdateCtx}}};

use super::camera::ManualCamera;

fn init_world(world: &mut World) {
    world.push(crate::world::camera::GetManualCamera);
}

pub struct Scene {
    pub final_position: BuffersAllocPosition,
    allocs: Vec<Position>,
    world: World,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");
        let mut world = World::new();
        init_world(&mut world);
        crate::content::build(&mut world);

        let mut alloc = BuffersAllocPosition::new();
        let allocs = world.allocs(&mut alloc);

        Scene {
            final_position: alloc,
            world,
            allocs,
        }
    }
    pub fn update(&mut self, registry: &mut PipelinesRegistry, queue: &wgpu::Queue, time: f32, cam: &ManualCamera) {
        // let _span = info_span!("update_scene").entered();
        // info!("Updating scene");
        let mut views = registry.views(queue);
        let ctx = WorldUpdateCtx {
            var_update: UpdateCtx {
                time,
            },
            allocs: &self.allocs,
            views: views.each_mut().map(|v| v.deref_mut()),
            cam: cam.cam,
        };
        self.world.update(ctx);
        registry.set_camera(queue, self.world.get_cam(cam.current_cam_idx).matrix(cam.aspect_ratio()))
    }
}
