use tracing::{info, info_span};
use wgpu::hal::auxil::db;
use crate::materials::shaders::{ShaderFile, Shaders};
use crate::materials::vertex::UniformTriangleVertex;
use crate::utils::macros::array_key;

pub struct PipeNames {
    pub base_name: &'static str,
    pub render_pipe_layout: &'static str,
    pub render_pipe: &'static str,
}
macro_rules! pipe_names {
    ($name: literal) => {
        crate::materials::pipelines::PipeNames {
            base_name: $name,
            render_pipe_layout: concat!("Render pipe layout: ", $name),
            render_pipe: concat!("Render pipe: ", $name),
        }
    };
}
pub(crate) use pipe_names;

array_key!(
    pub enum PipelineLabel {
        UniformTriangle,
    }
);
impl PipelineLabel {
    pub fn names(&self) -> PipeNames {
        match self {
            Self::UniformTriangle => pipe_names!("UniformTriangle"),
        }
    }
    pub fn vertex_file(&self) -> ShaderFile {
        match self {
            Self::UniformTriangle => ShaderFile::Simple,
        }
    }
    pub fn vertex_entry_point(&self) -> &'static str {
        match self {
            Self::UniformTriangle => "vs_main",
        }
    }
    pub fn fragment_file(&self) -> ShaderFile {
        match self {
            Self::UniformTriangle => ShaderFile::Simple,
        }
    }
    pub fn fragment_entry_point(&self) -> &'static str {
        match self {
            Self::UniformTriangle => "fs_main",
        }
    }
    pub fn vertex_attributes(&self) -> &'static [wgpu::VertexAttribute] {
        match self {
            Self::UniformTriangle => UniformTriangleVertex::ATTRS,
        }
    }
    pub fn vertex_size(&self) -> usize {
        match self {
            Self::UniformTriangle => 16,
        }
    }
}

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    // index_buffer: Option<wgpu::Buffer>,
    buffer_size: (usize, usize),
    label: PipelineLabel,
}
impl Pipeline {
    pub fn new(
        label: PipelineLabel,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        bindings_layout: &wgpu::BindGroupLayout,
        shaders: &Shaders,
        buffer_size: (usize, usize),
    ) -> Self {
        let _span = info_span!("pipeline").entered();
        let names = label.names();
        info!("Creating pipeline {}", names.base_name);
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(names.render_pipe_layout),
            bind_group_layouts: &[bindings_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(names.render_pipe),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &[
                    wgpu::VertexBufferLayout {
                        step_mode: wgpu::VertexStepMode::Vertex,
                        array_stride: label.vertex_size() as wgpu::BufferAddress,
                        attributes: label.vertex_attributes()
                    }
                ],
                module: shaders.get(label.vertex_file()),
                entry_point: label.vertex_entry_point(),
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: shaders.get(label.fragment_file()),
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex buffer"),
            size: buffer_size.0 as wgpu::BufferAddress,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            render_pipeline,
            // index_buffer: None,
            vertex_buffer,
            buffer_size,
            label,
        }
    }
    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..(self.buffer_size.0/self.label.vertex_size()) as u32, 0..1);
        // if let Some(index) = &self.index_buffer {
        //
        // } else {
        //
        // }
    }
    pub fn view<'a>(&'a self, queue: &'a wgpu::Queue) -> wgpu::QueueWriteBufferView<'a> {
        queue.write_buffer_with(
            &self.vertex_buffer,
            0,
            (self.buffer_size.0 as u64).try_into().unwrap(),
        ).unwrap()
    }
}
