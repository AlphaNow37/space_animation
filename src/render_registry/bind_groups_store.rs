use std::array::from_fn;
use tracing::{info, info_span};
use crate::world::world::{GlobalStore, World};

const NB_STORE: usize = GlobalStore::COUNT;

pub struct StoreBindings {
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; NB_STORE],
}
impl StoreBindings {
    pub fn new(device: &wgpu::Device, sizes: [usize; NB_STORE]) -> Self {
        let _span = info_span!("store_bindings").entered();
        info!("Creating store bindings");
        info!("Requested sizes: {:?}", sizes);
        let viss = World::store_stages();
        let names = World::store_name();
        let struct_sizes = World::store_sized();
        let layout_entries: [_; NB_STORE] = from_fn(|idx|
            wgpu::BindGroupLayoutEntry {
                binding: idx as u32,
                count: None,
                visibility: viss[idx],
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {read_only: true},
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }
        );
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Store bind group layout"),
            entries: &layout_entries,
        });
        dbg!(sizes, struct_sizes);
        let buffers: [_; NB_STORE] = from_fn(|idx|
            device.create_buffer(&wgpu::BufferDescriptor {
                size:  (struct_sizes[idx] * (sizes[idx] + 1)) as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
                label: Some(&format!("Buffer of {}", names[idx]))
            })
        );
        let entries: [_; NB_STORE] = from_fn(|idx| {
            wgpu::BindGroupEntry {
                binding: idx as u32,
                resource: buffers[idx].as_entire_binding()
            }
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Store bind group"),
            entries: &entries,
            layout: &layout,
        });
        info!("Succesfully created {} store bindings", NB_STORE);
        Self {
            layout,
            bind_group,
            buffers,
        }
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }
    pub fn views<'a>(&'a self, queue: &'a wgpu::Queue) -> [Option<wgpu::QueueWriteBufferView>; NB_STORE] {
        self.buffers
            .each_ref()
            .map(|buf|
                wgpu::BufferSize::try_from(buf.size())
                    .ok()
                    .and_then(|size| queue.write_buffer_with(buf, 0, size))
            )
    }
}
