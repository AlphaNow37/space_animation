use crate::world::world_builder::WorldId;
use crate::{
    render_registry::{alloc::BufferAllocator, registry::PipelinesRegistry},
    world::{
        world::{World, WorldUpdateCtx},
        world_builder::WorldsBuilder,
    },
};
use tracing::{info, info_span};
use crate::world::world::{Worlds, WorldSettings};

use super::camera::ManualCamera;

pub struct Scene {
    worlds: Vec<World>,
    id_by_layer: Vec<Vec<WorldId>>,
    pub allocs: Vec<BufferAllocator>,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");

        let mut worlds_builder = crate::content::build();

        worlds_builder.finalize();

        let allocs = worlds_builder.get_buffer_allocs();
        let (worlds, id_by_layer) = worlds_builder.to_run_worlds();

        Scene {
            worlds,
            allocs,
            id_by_layer,
        }
    }
    pub fn update(
        &mut self,
        registry: &mut PipelinesRegistry,
        queue: &wgpu::Queue,
        time: f32,
        cam: &ManualCamera,
    ) {
        // let _span = info_span!("update_scene").entered();
        // info!("Updating scene");

        let empty_w = World::new();
        let mut worlds = Worlds {
            world: &empty_w,
            worlds: &self.worlds,
            settings: WorldSettings {
                base_time: time,
                cam_settings: cam.cam,
            }
        };

        for ids in &self.id_by_layer {
            for id in ids {
                let i = id.get();
                let w = &self.worlds[i];
                worlds.world = w;
                let mut instance_views = registry.views(queue, i);
                let mut store_views = registry.store_bindings[i].views(queue);
                let ctx = WorldUpdateCtx {
                    instance_bufs: instance_views.each_mut().map(|r| {
                        r.each_mut().map(|view_opt| {
                            view_opt
                                .as_mut()
                                .map(|view| bytemuck::cast_slice_mut(&mut *view))
                                .unwrap_or(&mut [])
                        })
                    }),
                    stores: store_views.each_mut().map(|v| {
                        v.as_deref_mut()
                            .map(|view| bytemuck::cast_slice_mut(view))
                            .unwrap_or(&mut [])
                    }),
                    worlds: &worlds,
                };

                w.update(ctx);
            }
        }
        let wcam = self.worlds.last().unwrap().get_cam(cam.current_cam_idx);
        registry
            .base_bindings
            .set_camera(queue, wcam.matrix(cam.aspect_ratio()));
        registry
            .base_bindings
            .set_camera_transform(queue, wcam.pos.to_mat4());
    }
}
