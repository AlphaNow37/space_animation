use std::num::NonZeroU64;
use tracing::{info_span, info};
use crate::math::Mat4;
use crate::render_registry::bind_group_base::BaseBindings;
use crate::render_registry::bind_groups_store::StoreBindings;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::pipelines::Pipeline;
use crate::render_registry::shaders::Shaders;
use crate::render_registry::vertex::VertexType;

use super::alloc::BufferAllocator;
use super::depth::DepthBuffer;

pub const PIPELINE_COUNT: usize = VertexType::COUNT * MaterialType::COUNT;

pub struct PipelinesRegistry {
    pub base_bindings: BaseBindings,
    pub store_bindings: StoreBindings,
    depth_buffer: DepthBuffer,
    // shaders: Shaders,
    pub pipes: [[Option<Pipeline>; MaterialType::COUNT]; VertexType::COUNT],
}
impl PipelinesRegistry {
    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration, alloc: &BufferAllocator) -> Self {
        let _span = info_span!("registry").entered();
        let base_bindings = BaseBindings::new(device);
        let store_bindings = StoreBindings::new(device, alloc);
        let shaders = Shaders::new(device);
        let depth_buffer = DepthBuffer::new(device, surf_config);

        let pipes = VertexType::ARRAY
            .map(|vertex| MaterialType::ARRAY
                .map(|material| {
                    let nb_instance = alloc.get_instance_count(vertex, material);
                    NonZeroU64::new(nb_instance as u64)
                        .map(|size| Pipeline::new(
                            vertex,
                            material,
                            device,
                            surf_config,
                            &base_bindings.layout,
                            &store_bindings.layout,
                            &shaders,
                            size
                        ))
                })
            );

        info!("Succesfully created {} pipelines", PIPELINE_COUNT);
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

        for r in &self.pipes {
            for maybe_pipe in r {
                if let Some(pipe) = maybe_pipe {
                    pipe.render(&mut render_pass)
                }
            }
        }
    }
    pub fn views<'a>(&'a self, queue: &'a wgpu::Queue) -> [[Option<wgpu::QueueWriteBufferView<'a>>; MaterialType::COUNT]; VertexType::COUNT] {
        self.pipes
            .each_ref()
            .map(|row|
                row
                    .each_ref()
                    .map(|maybe_pipe|
                        maybe_pipe
                            .as_ref()
                            .map(|pipe|
                                pipe.view_instance(queue)
                            )
                    )
            )
    }
}
