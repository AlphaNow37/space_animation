use log::info;
use tracing::info_span;
// use crate::app::App;
use crate::materials::{dispatcher::BuffersAllocPosition, pipelines::PipelineLabel, registry::PipelinesRegistry};

pub struct Scene {
    pub final_position: BuffersAllocPosition,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");
        let mut pos = BuffersAllocPosition::new();
        pos.alloc(PipelineLabel::UniformTriangle, 5, 0);
        Scene {
            final_position: pos,
        }
    }
    pub fn update(&self, registry: &mut PipelinesRegistry, queue: &wgpu::Queue, time: f32) {
        let _span = info_span!("update_scene").entered();
        info!("Updating scene");
    }
}
