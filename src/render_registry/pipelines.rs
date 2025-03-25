use crate::render_registry::depth::DepthBuffer;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::prefabs::VertexPoss;
use crate::render_registry::shaders::Shaders;
use crate::render_registry::vertex::{AuxiliaryBufferDesc, VertexType};
use std::num::NonZeroU64;
use tracing::{info, info_span};
use wgpu::util::DeviceExt;

enum AuxiliaryBuffer {
    VertexPoss(wgpu::Buffer),
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    aux_buffers: Vec<AuxiliaryBuffer>,
    nb_instance: NonZeroU64,
    vertex: VertexType,
}
impl Pipeline {
    pub fn new(
        vertex: VertexType,
        material: MaterialType,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        base_bindings_layout: &wgpu::BindGroupLayout,
        store_bindings_layout: &wgpu::BindGroupLayout,
        shaders: &Shaders,
        nb_instance: NonZeroU64,
    ) -> Self {
        let _span = info_span!("pipeline").entered();
        let name = format!("{}:{}", vertex.name(), material.name());
        info!("Creating pipeline {name}");
        let instance_label = vertex.instance_buffer_label();

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("Pipeline layout {name}")),
                bind_group_layouts: &[base_bindings_layout, store_bindings_layout],
                push_constant_ranges: &[],
            });

        let mut buffers_descriptor = vec![wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Instance,
            array_stride: instance_label.elt_size() as wgpu::BufferAddress,
            attributes: instance_label.attrs(),
        }];

        let mut aux_buffers = Vec::new();
        for aux_buf_label in vertex.aux_buffers() {
            match aux_buf_label {
                AuxiliaryBufferDesc::VertexPoss(VertexPoss { label, content, .. }) => {
                    buffers_descriptor.push(wgpu::VertexBufferLayout {
                        step_mode: wgpu::VertexStepMode::Vertex,
                        array_stride: label.elt_size() as wgpu::BufferAddress,
                        attributes: label.attrs(),
                    });
                    aux_buffers.push(AuxiliaryBuffer::VertexPoss(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(&format!("Auxiliary buffer {name}")),
                            usage: wgpu::BufferUsages::VERTEX,
                            contents: bytemuck::cast_slice(&content),
                        },
                    )));
                }
            }
        }
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("render pipeline {name}")),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &buffers_descriptor,
                module: shaders.get(),
                entry_point: Some(vertex.entry_point()),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shaders.get(),
                entry_point: Some(material.entry_point()),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                bias: wgpu::DepthBiasState::default(),
                depth_compare: wgpu::CompareFunction::LessEqual,
                depth_write_enabled: true,
                format: DepthBuffer::FORMAT,
                stencil: wgpu::StencilState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("Instance buffer {name}")),
            size: instance_label.elt_size() * nb_instance.get(),
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        Self {
            render_pipeline,
            aux_buffers,
            instance_buffer,
            nb_instance,
            vertex,
        }
    }
    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        for (i, aux_buffer) in self.aux_buffers.iter().enumerate() {
            match &aux_buffer {
                AuxiliaryBuffer::VertexPoss(buffer) => {
                    render_pass.set_vertex_buffer((i + 1) as u32, buffer.slice(..));
                }
            }
        }
        render_pass.draw(0..self.vertex.nb_vertex(), 0..self.nb_instance.get() as u32);
    }

    pub fn view_instance<'a>(&'a self, queue: &'a wgpu::Queue) -> wgpu::QueueWriteBufferView<'a> {
        let nb_inst = self.nb_instance.get();
        let elt_size = self.vertex.instance_buffer_label().elt_size();
        let size = NonZeroU64::new(nb_inst * elt_size).expect("Vertex size should not be zero");
        queue
            .write_buffer_with(&self.instance_buffer, 0, size)
            .expect("Failed to write the buffer!")
    }
}
