use crate::math::Transform;
use crate::utils::{Length, binary_search_interval};
use crate::world::primitives::camera::Camera;
use crate::world::world::{WorldSettings, Worlds};
use crate::world::world_builder::{WorldBuilderFinalizationValue, WorldId};
use crate::{
    render_registry::{alloc::BufferAllocator, registry::PipelinesRegistry},
    world::{world::World, world_builder::WorldsBuilder},
};
use tracing::{info, info_span};

use super::camera::ManualCamera;
const BUCKET_FADE_THRESOLD: f32 = 5.;
const VIEW_MAX_DISTANCE: f32 = 100.;

fn view_parameters(cam_pos: Transform, world_extents: Option<Transform>) -> (bool, f32) {
    match world_extents {
        None => (true, 1.),
        Some(extents) => {
            // TODO passer dans l'espace de la camera ?
            let size_squared = 0.0f32
                .max(extents.x().length_squared())
                .max(extents.y().length_squared())
                .max(extents.z().length_squared());
            let dist = ((cam_pos.trans() - extents.trans()).length_squared() / size_squared).sqrt();
            (
                dist < VIEW_MAX_DISTANCE,
                BUCKET_FADE_THRESOLD / dist.max(BUCKET_FADE_THRESOLD),
            )
        }
    }
}

pub struct Scene {
    worlds: Vec<World>,
    ticks: Vec<f32>,
    id_by_layer: Vec<Vec<WorldId>>,
    pub allocs: Vec<BufferAllocator>,
    camera_offsets: Vec<usize>,
}
impl Scene {
    pub fn new(builder_fun: &mut dyn FnMut() -> WorldsBuilder) -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");

        let mut worlds_builder = builder_fun();

        let WorldBuilderFinalizationValue {
            camera_offsets,
            worlds,
            buffer_allocations,
            id_by_layer,
        } = worlds_builder.finalize();

        Scene {
            ticks: vec![1.; worlds.len()],
            worlds,
            allocs: buffer_allocations,
            id_by_layer,
            camera_offsets,
        }
    }
    fn get_cam(&self, id: isize) -> Camera {
        let (world_id, cam_idx) = binary_search_interval(
            &self.camera_offsets,
            id.rem_euclid(*self.camera_offsets.last().unwrap() as isize) as usize,
        );
        self.worlds[world_id].get_cam(cam_idx)
    }
    pub fn update(
        &mut self,
        registry: &mut PipelinesRegistry,
        queue: &wgpu::Queue,
        time: f32,
        manu_cam: &ManualCamera,
    ) {
        // let _span = info_span!("update_scene").entered();
        // info!("Updating scene");

        let empty_w = World::new();
        let mut worlds = Worlds {
            world: &empty_w,
            worlds: &self.worlds,
            settings: WorldSettings {
                base_time: time,
                cam_settings: manu_cam.cam,
            },
        };
        let wcam = self.get_cam(manu_cam.current_cam_idx);
        for ids in &self.id_by_layer {
            for id in ids {
                let i = id.get();
                let w = &self.worlds[i];
                worlds.world = w;

                let world_extents = w.view_bounding_box.as_ref().map(|v| v.update(&worlds));
                let (show, tick_add) = view_parameters(wcam.pos, world_extents);

                self.ticks[i] += tick_add;
                if self.ticks[i] >= 1. {
                    self.ticks[i] = 0.;
                    w.update_registers(&worlds);
                    let mut stores_views = registry.store_bindings[i].views(queue);
                    let stores = stores_views.each_mut().map(|v| {
                        v.as_deref_mut()
                            .map(|view| bytemuck::cast_slice_mut(view))
                            .unwrap_or(&mut [])
                    });
                    w.write_stores(stores);
                }
                if show {
                    let mut instance_bufs_views = registry.views(queue, i);
                    let instance_bufs = instance_bufs_views.each_mut().map(|r| {
                        r.each_mut().map(|view_opt| {
                            view_opt
                                .as_mut()
                                .map(|view| bytemuck::cast_slice_mut(&mut *view))
                                .unwrap_or(&mut [])
                        })
                    });
                    w.redraw(instance_bufs);
                }
                registry.pipes[i].activated = show;
            }
        }
        let wcam = self.get_cam(manu_cam.current_cam_idx);
        registry
            .base_bindings
            .set_camera(queue, wcam.matrix(manu_cam.aspect_ratio()));
        registry
            .base_bindings
            .set_camera_transform(queue, wcam.pos.to_mat4());
    }
}
