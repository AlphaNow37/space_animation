use std::num::NonZeroU64;
use tracing::{info, info_span};
use wgpu::util::DeviceExt;
use crate::render_registry::depth::DepthBuffer;
use crate::render_registry::materials::MaterialType;
use crate::render_registry::prefabs::VertexPoss;
use crate::render_registry::shaders::Shaders;
use crate::render_registry::vertex::{SecondaryBufferDesc, VertexType};

enum SecondaryBuffer {
    VertexPoss(wgpu::Buffer),
    None,
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    secondary_buffer: SecondaryBuffer,
    nb_instance: NonZeroU64,
    material: MaterialType,
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
        let secondary_label = vertex.secondary_buffer();

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline layout {name}")),
            bind_group_layouts: &[base_bindings_layout, store_bindings_layout],
            push_constant_ranges: &[],
        });

        let mut buffers_descriptor = vec![wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Instance,
            array_stride: instance_label.elt_size() as wgpu::BufferAddress,
            attributes: instance_label.attrs()
        }];
        match secondary_label {
            SecondaryBufferDesc::VertexPoss(VertexPoss {label, ..}) => buffers_descriptor.push(
                wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    array_stride: label.elt_size() as wgpu::BufferAddress,
                    attributes: label.attrs(),
                }
            ),
            SecondaryBufferDesc::None => {},
        };
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("render pipeline {name}")),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &buffers_descriptor,
                module: shaders.get(),
                entry_point: vertex.entry_point(),
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: shaders.get(),
                entry_point: material.entry_point(),
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
        let secondary_buffer = match secondary_label {
            SecondaryBufferDesc::None => SecondaryBuffer::None,
            SecondaryBufferDesc::VertexPoss(VertexPoss { content, ..}) => SecondaryBuffer::VertexPoss(
                device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some(&format!("Vertex poss buffer {name}")),
                        usage: wgpu::BufferUsages::VERTEX,
                        contents: bytemuck::cast_slice(&content),
                    },
                )
            ),
        };
        Self {
            render_pipeline,
            secondary_buffer,
            instance_buffer,
            nb_instance,
            vertex,
            material,
        }
    }
    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        match &self.secondary_buffer {
            SecondaryBuffer::VertexPoss(buffer) => {
                render_pass.set_vertex_buffer(1, buffer.slice(..));
            },
            SecondaryBuffer::None => {},
        }
        render_pass.draw(0..self.vertex.nb_vertex(), 0..self.nb_instance.get() as u32);
    }

    pub fn view_instance<'a>(&'a self, queue: &'a wgpu::Queue) -> wgpu::QueueWriteBufferView<'a> {
        let nb_inst = self.nb_instance.get();
        let elt_size = self.vertex.instance_buffer_label().elt_size();
        let size = NonZeroU64::new(nb_inst * elt_size).expect("Vertex size should not be zero");
        queue.write_buffer_with(&self.instance_buffer, 0, size).expect("Failed to write the buffer!")
    }
}
