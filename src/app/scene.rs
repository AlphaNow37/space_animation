use log::info;
use tracing::info_span;
// use crate::app::App;
use crate::materials::registry::PipelinesRegistry;

struct Scene {

}
impl Scene {
    pub fn new() -> Self {
        Scene {}
    }
    pub fn update(&self, registry: &mut PipelinesRegistry, queue: &wgpu::Queue, time: f32) {

    }
}

pub fn recreate_scene(registry: &mut PipelinesRegistry) {
    let _span = info_span!("scene").entered();
    info!("Creating scene");
}
