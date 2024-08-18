use bytemuck::NoUninit;
use tracing::{info, info_span};
use crate::utils::macros::array_key;

array_key!(
    pub enum EntryType {
        Time,
        Camera,
    }
);

impl EntryType {
    fn visibility(&self) -> wgpu::ShaderStages {
        use wgpu::ShaderStages;
        match self {
            Self::Time => ShaderStages::FRAGMENT,
            Self::Camera => ShaderStages::VERTEX,
        }
    }
    fn min_size(&self) -> u64 {
        4*match self {
            Self::Time => 1,
            Self::Camera => 16,
        }
    }
}

pub struct Bindings {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; EntryType::COUNT],
}
impl Bindings {
    pub fn new(device: &wgpu::Device) -> Self {
        let _span = info_span!("bindings").entered();
        info!("Creating bindings");
        let layout_entries = EntryType::ARRAY
            .map(|e| wgpu::BindGroupLayoutEntry {
                binding: e as u32,
                count: None,
                visibility: e.visibility(),
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &layout_entries,
        });
        let buffers = EntryType::ARRAY
            .map(|e| device.create_buffer(&wgpu::BufferDescriptor {
                size: e.min_size().next_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
                label: Some(&format!("Buffer of {}", e.name()))
            }));
        let entries = EntryType::ARRAY
            .map(|e| wgpu::BindGroupEntry {
                binding: e as u32,
                resource: buffers[e as usize].as_entire_binding()
            });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            entries: &entries,
            layout: &layout,
        });
        info!("Succesfully created {} bindings", EntryType::COUNT);
        Self {
            layout,
            bind_group,
            buffers,
        }
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
    pub fn write(&self, queue: &wgpu::Queue, entry: EntryType, value: &[impl NoUninit]) {
        queue.write_buffer(
            &self.buffers[entry as usize],
            0,
            bytemuck::cast_slice(value),
        )
    }
}
