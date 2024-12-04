use tracing::{info_span, info};
use crate::{
    render_registry::{
        alloc::BufferAllocator,
        registry::PipelinesRegistry,
    },
    world::world::{World, WorldUpdateCtx},
};

use super::camera::ManualCamera;

fn init_world(world: &mut World) {
    world.push(crate::world::primitives::camera::GetManualCamera);
}

pub struct Scene {
    pub alloc: BufferAllocator,
    world: World,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");
        let mut world = World::new();
        init_world(&mut world);
        crate::content::build(&mut world);

        let mut alloc = BufferAllocator::new();
        world.allocs(&mut alloc);

        Scene {
            alloc,
            world,
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
        let mut instance_views = registry.views(queue);
        let mut store_views = registry.store_bindings.views(queue);
        let ctx = WorldUpdateCtx {
            instance_bufs: instance_views.each_mut()
                .map(|r|
                    r.each_mut()
                        .map(|view_opt|
                            view_opt
                                .as_mut()
                                .map(|view|
                                    bytemuck::cast_slice_mut(&mut *view)).unwrap_or(&mut [])
                        )
                ),
            stores: store_views.each_mut()
                .map(|v|
                    v.as_deref_mut()
                        .map(|view| bytemuck::cast_slice_mut(view))
                        .unwrap_or(&mut [])
                ),
            cam: cam.cam,
            time,
        };
        self.world.update(ctx);
        let wcam = self.world.get_cam(cam.current_cam_idx);
        registry.base_bindings.set_camera(queue, wcam.matrix(cam.aspect_ratio()));
        registry.base_bindings.set_camera_transform(queue, wcam.pos.to_mat4());
    }
}
