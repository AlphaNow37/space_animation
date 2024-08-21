use tracing::{info, info_span};
use wgpu::util::DeviceExt;
use crate::render_registry::depth::DepthBuffer;
use crate::render_registry::prefabs::CIRCLE_POS;
use crate::render_registry::shaders::Shaders;
use crate::render_registry::vertex::{PosVertex, SphereVertexCol1, TriVertexCol1, TriVertexCol2};
use crate::utils::macros::array_key;
use super::vertex::VertexLike;

pub struct PipeNames {
    pub base_name: &'static str,
    pub render_pipe_layout: &'static str,
    pub render_pipe: &'static str,
    pub index_buffer: &'static str,
    pub vertex_buffer: &'static str,
    pub instance_aux_buffer: &'static str,
}
macro_rules! pipe_names {
    ($name: literal) => {
        crate::render_registry::pipelines::PipeNames {
            base_name: $name,
            render_pipe_layout: concat!("Render pipe layout: ", $name),
            render_pipe: concat!("Render pipe: ", $name),
            index_buffer: concat!("Index buffer: ", $name),
            vertex_buffer: concat!("Vertex buffer: ", $name),
            instance_aux_buffer: concat!("Instance auxilliary buffer: ", $name)
        }
    };
}

enum VertexBufferLabel {
    TriCol1,
    TriCol2,
    SphereCol1,
    InstTriVertex,
}
impl VertexBufferLabel {
    fn elt_size(&self) -> usize {
        match self {
            Self::TriCol1 => TriVertexCol1::SIZE,
            Self::TriCol2 => TriVertexCol2::SIZE,
            Self::SphereCol1 => SphereVertexCol1::SIZE,
            Self::InstTriVertex => PosVertex::SIZE,
        }
    }
    fn attrs(&self) -> &'static [wgpu::VertexAttribute] {
        match self {
            Self::TriCol1 => TriVertexCol1::ATTRS,
            Self::TriCol2 => TriVertexCol2::ATTRS,
            Self::SphereCol1 => SphereVertexCol1::ATTRS,
            Self::InstTriVertex => PosVertex::ATTRS,
        }
    }
}

array_key!(
    pub enum PipelineLabel {
        UniformTriangle,
        SpongeTriangle,
        UniformSphere,
    }
);
impl PipelineLabel {
    fn names(&self) -> PipeNames {
        match self {
            Self::UniformTriangle => pipe_names!("UniformTriangle"),
            Self::SpongeTriangle => pipe_names!("SpongeTriangle"),
            Self::UniformSphere => pipe_names!("UniformSphere"),
        }
    }
    fn vertex_entry_point(&self) -> &'static str {
        match self {
            Self::UniformTriangle => "vs_tri_color1",
            Self::SpongeTriangle => "vs_tri_color2",
            Self::UniformSphere => "vs_sphere_color1",
        }
    }
    fn fragment_entry_point(&self) -> &'static str {
        match self {
            Self::UniformTriangle => "fs_uniform",
            Self::SpongeTriangle => "fs_sponge",
            Self::UniformSphere => "fs_uniform",
        }
    }
    fn vertex_buffer_label(&self) -> VertexBufferLabel {
        match self {
            Self::UniformTriangle => VertexBufferLabel::TriCol1,
            Self::SpongeTriangle => VertexBufferLabel::TriCol2,
            Self::UniformSphere => VertexBufferLabel::SphereCol1,
        }
    }
    fn vertex_aux_buffer_label(&self) -> Option<VertexBufferLabel> {
        match self {
            Self::UniformSphere => Some(VertexBufferLabel::InstTriVertex),
            _ => None,
        }
    }
    fn vertex_aux_buffer_content(&self) -> Option<(u32, &'static [u32])> {
        match self {
            Self::UniformSphere => Some(*CIRCLE_POS),
            _ => None,
        }
    }
}

enum SecondaryBuffer {
    InstanceAux(wgpu::Buffer, u32),
    Index(wgpu::Buffer),
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    secondary_buffer: SecondaryBuffer,
    buffer_count: (usize, usize),
    label: PipelineLabel,
}
impl Pipeline {
    pub fn new(
        label: PipelineLabel,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bindings_layout: &wgpu::BindGroupLayout,
        shaders: &Shaders,
        buffer_count: (usize, usize),
    ) -> Self {
        let _span = info_span!("pipeline").entered();
        let names = label.names();
        info!("Creating pipeline {}", names.base_name);
        let vertex_label = label.vertex_buffer_label();
        let vertex_aux_label = label.vertex_aux_buffer_label();

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(names.render_pipe_layout),
            bind_group_layouts: &[bindings_layout],
            push_constant_ranges: &[],
        });
        let buffers_descriptor = match vertex_aux_label {
            None => &[
                wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    array_stride: vertex_label.elt_size() as wgpu::BufferAddress,
                    attributes: vertex_label.attrs()
                }
            ] as &[wgpu::VertexBufferLayout],
            Some(ref l) => &[
                wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Instance,
                    array_stride: vertex_label.elt_size() as wgpu::BufferAddress,
                    attributes: vertex_label.attrs()
                },
                wgpu::VertexBufferLayout {
                    step_mode: wgpu::VertexStepMode::Vertex,
                    array_stride: l.elt_size() as wgpu::BufferAddress,
                    attributes: l.attrs(),
                }
            ] as &[wgpu::VertexBufferLayout],
        };
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(names.render_pipe),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: buffers_descriptor,
                module: shaders.get(),
                entry_point: label.vertex_entry_point(),
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: shaders.get(),
                entry_point: label.fragment_entry_point(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
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

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(names.vertex_buffer),
            size: (vertex_label.elt_size() * buffer_count.0) as wgpu::BufferAddress,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let secondary_buffer = match vertex_aux_label {
            None => SecondaryBuffer::Index(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(names.index_buffer),
                size: 4*buffer_count.1 as wgpu::BufferAddress,
                mapped_at_creation: false,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            })),
            Some(_) => {
                debug_assert_eq!(buffer_count.1, 0, "A non-indexed pipeline should not contain indices");
                let (len, content) = label.vertex_aux_buffer_content().expect("There should be an initial content");
                SecondaryBuffer::InstanceAux(device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some(names.instance_aux_buffer),
                        usage: wgpu::BufferUsages::VERTEX,
                        contents: bytemuck::cast_slice(&content),
                    },
                ), len)
            }
        };
        Self {
            render_pipeline,
            secondary_buffer,
            vertex_buffer,
            buffer_count,
            label,
        }
    }
    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        match &self.secondary_buffer {
            SecondaryBuffer::Index(index_buffer) => {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..self.buffer_count.1 as u32, 0, 0..1);
            },
            SecondaryBuffer::InstanceAux(aux_buffer, nb_vertex) => {
                render_pass.set_vertex_buffer(1, aux_buffer.slice(..));
                render_pass.draw(0..*nb_vertex, 0..self.buffer_count.0 as u32);
            }
        }
    }

    pub fn view_vertex<'a>(&'a self, queue: &'a wgpu::Queue) -> Option<wgpu::QueueWriteBufferView<'a>> {
        let size = (self.label.vertex_buffer_label().elt_size() * self.buffer_count.0) as u64;
        let buffer_size = wgpu::BufferSize::new(size)?;
        queue.write_buffer_with(&self.vertex_buffer, 0, buffer_size)
    }
    pub fn view_index<'a>(&'a self, queue: &'a wgpu::Queue) -> Option<wgpu::QueueWriteBufferView<'a>> {
        let SecondaryBuffer::Index(buffer) = &self.secondary_buffer else {return None};
        let size = 4 * self.buffer_count.1 as u64;
        let buffer_size = wgpu::BufferSize::new(size)?;
        queue.write_buffer_with(buffer, 0, buffer_size)
    }
}
