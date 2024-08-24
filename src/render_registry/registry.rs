use tracing::{info_span, info};
use crate::math::Mat4;
use crate::render_registry::bind_group_base::BaseBindings;
use crate::render_registry::bind_groups_store::StoreBindings;
use crate::render_registry::pipelines::{Pipeline, PipelineLabel};
use crate::render_registry::shaders::Shaders;
use crate::world::world::GlobalStore;

use super::alloc::BufferAllocator;
use super::depth::DepthBuffer;

pub struct PipelinesRegistry {
    pub base_bindings: BaseBindings,
    pub store_bindings: StoreBindings,
    depth_buffer: DepthBuffer,
    // shaders: Shaders,
    pub pipes: [Pipeline; PipelineLabel::COUNT],
}
impl PipelinesRegistry {
    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration, pos: &BufferAllocator) -> Self {
        let _span = info_span!("registry").entered();
        let base_bindings = BaseBindings::new(device);
        let store_bindings = StoreBindings::new(device, pos.store);
        let shaders = Shaders::new(device);
        let depth_buffer = DepthBuffer::new(device, surf_config);
        let pipes = PipelineLabel::ARRAY
            .map(|label| Pipeline::new(
                label, 
                device, 
                surf_config,
                &base_bindings.layout,
                &store_bindings.layout,
                &shaders,
                pos.get_count(label) as u32
            ));
        info!("Succesfully created {} pipelines", pipes.len());
        Self {
            base_bindings,
            store_bindings,
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
        self.base_bindings.put(&mut render_pass);
        self.store_bindings.put(&mut render_pass);

        for pipe in &self.pipes {
            pipe.render(&mut render_pass)
        }
    }
    pub fn views<'a>(&'a self, queue: &'a wgpu::Queue) -> [Option<wgpu::QueueWriteBufferView<'a>>; PipelineLabel::COUNT] {
        PipelineLabel::ARRAY
            .map(|label| {
                self.pipes[label as usize].view_instance(queue)
            })
    }
}
