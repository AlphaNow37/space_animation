use tracing::info_span;
use log::info;
use crate::materials::bind_groups::{Bindings, EntryType};
use crate::materials::pipelines::{Pipeline, PipelineLabel};
use crate::materials::shaders::Shaders;

use super::alloc::BuffersAllocPosition;
use super::depth::DepthBuffer;

pub struct PipelinesRegistry {
    bindings: Bindings,
    depth_buffer: DepthBuffer,
    // shaders: Shaders,
    pub pipes: [Pipeline; PipelineLabel::COUNT],
}
impl PipelinesRegistry {
    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration, pos: &BuffersAllocPosition) -> Self {
        let _span = info_span!("registry").entered();
        let bindings = Bindings::new(device);
        let shaders = Shaders::new(device);
        let depth_buffer = DepthBuffer::new(device, surf_config);
        let pipes = PipelineLabel::ARRAY
            .map(|label| Pipeline::new(
                label, 
                device, 
                surf_config,
                 &bindings.layout,
                  &shaders,
                  pos.get_bytes(label)
                ));
        info!("Succesfully created {} pipelines", pipes.len());
        Self {
            bindings,
            // shaders,
            pipes,
            depth_buffer,
        }
    }
    pub fn on_resize(&mut self, device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration) {
        self.depth_buffer = DepthBuffer::new(device, surf_config);
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
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
    pub fn views<'a>(&'a self, queue: &'a wgpu::Queue) -> [wgpu::QueueWriteBufferView<'a>; PipelineLabel::COUNT] {
        PipelineLabel::ARRAY
            .map(|label| self.pipes[label as usize].view(&queue))
    }
}
