use tracing::{info_span, info};
use crate::math::Mat4;
use crate::render_registry::bind_groups::{Bindings, EntryType};
use crate::render_registry::pipelines::{Pipeline, PipelineLabel};
use crate::render_registry::shaders::Shaders;

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
                pos.get_count(label)
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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
    pub fn set_time(&self, queue: &wgpu::Queue, time: f32){
        self.bindings.write(queue, EntryType::Time, &[time])
    }
    pub fn set_camera(&self, queue: &wgpu::Queue, matrix: Mat4) {
        self.bindings.write(queue, EntryType::Camera, &matrix.to_array());
    }
    pub fn set_camera_transform(&self, queue: &wgpu::Queue, matrix: Mat4) {
        self.bindings.write(queue, EntryType::CameraTransform, &matrix.to_array());
    }
    pub fn views<'a>(&'a self, queue: &'a wgpu::Queue) -> [[Option<wgpu::QueueWriteBufferView<'a>>; 2]; PipelineLabel::COUNT] {
        PipelineLabel::ARRAY
            .map(|label| {
                let pipe = &self.pipes[label as usize];
                [pipe.view_vertex(queue), pipe.view_index(queue)]
            })
    }
}
