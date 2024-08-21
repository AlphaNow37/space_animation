use std::ops::DerefMut;

use tracing::{info_span, info};
use crate::{
    render_registry::{
        alloc::BuffersAllocPosition,
        registry::PipelinesRegistry,
    },
    world::{
        variators::variator::UpdateCtx,
        world::{World, WorldUpdateCtx},
    },
};
use crate::render_registry::mesh_builder::MeshBuilders;

use super::camera::ManualCamera;

fn init_world(world: &mut World) {
    world.push(crate::world::primitives::camera::GetManualCamera);
}

pub struct Scene {
    pub final_position: BuffersAllocPosition,
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
        world.allocs(&mut alloc);

        Scene {
            final_position: alloc,
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
        let mut views = registry.views(queue);
        let ctx = WorldUpdateCtx {
            var_update: UpdateCtx { time },
            builders: MeshBuilders::from_views(
                views.each_mut()
                    .map(|views|
                         views.each_mut()
                             .map(|v|
                                 v.as_deref_mut().map(|buf| bytemuck::cast_slice_mut(buf)).unwrap_or(&mut [])
                             )
                    )
            ),
            cam: cam.cam,
        };
        self.world.update(ctx);
        let wcam = self.world.get_cam(cam.current_cam_idx);
        registry.set_camera(queue, wcam.matrix(cam.aspect_ratio()));
        registry.set_camera_transform(queue, wcam.pos.to_mat4());
    }
}
