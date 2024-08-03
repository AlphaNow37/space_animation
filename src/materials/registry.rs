use tracing::info_span;
use log::info;
use crate::materials::bind_groups::{Bindings, EntryType};
use crate::materials::pipelines::{Pipeline, PipelineLabel};
use crate::materials::shaders::Shaders;

pub struct PipelinesRegistry {
    bindings: Bindings,
    // shaders: Shaders,
    pub pipes: [Pipeline; PipelineLabel::COUNT],
}
impl PipelinesRegistry {
    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration) -> Self {
        let _span = info_span!("registry").entered();
        let bindings = Bindings::new(device);
        let shaders = Shaders::new(device);
        let pipes = PipelineLabel::ARRAY
            .map(|label| Pipeline::new(label, device, surf_config, &bindings.layout, &shaders));
        info!("Succesfully created {} pipelines", pipes.len());
        Self {
            bindings,
            // shaders,
            pipes,
        }
    }
    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                }
            })],
            ..Default::default()
        });
        self.bindings.put(&mut render_pass);
        for pipe in &self.pipes {
            pipe.render(&mut render_pass)
        }
    }
    pub fn set_time(&self, queue: &wgpu::Queue, time: f32, loop_time: f32) {
        self.bindings.write(queue, EntryType::Time, &[time, loop_time])
    }
}
