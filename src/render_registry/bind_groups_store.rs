use crate::render_registry::alloc::BufferAllocator;
use crate::world::primitives::StoreLabel;
use tracing::{info, info_span};

pub struct StoreBindings {
    pub bind_group: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; StoreLabel::COUNT],
}
impl StoreBindings {
    pub fn new(
        device: &wgpu::Device,
        allocs: &[BufferAllocator],
    ) -> (Vec<Self>, wgpu::BindGroupLayout) {
        let _span = info_span!("store_bindings").entered();
        info!("Creating store bindings");
        info!("Requested sizes:");

        let mut occs = StoreLabel::ARRAY.map(|label| format!("{}: ", label.name()));
        for alloc in allocs {
            for label in StoreLabel::ARRAY {
                occs[label as usize].push_str(&format!(" {}", alloc.get_store_count(label)));
            }
        }
        for label in StoreLabel::ARRAY {
            info!("{}", occs[label as usize]);
        }

        let layout_entries = StoreLabel::ARRAY.map(|label| wgpu::BindGroupLayoutEntry {
            binding: label.bind(),
            count: None,
            visibility: label.stage(),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Store bind group layout"),
            entries: &layout_entries,
        });

        let bindings = allocs
            .iter()
            .map(|alloc| {
                let buffers = StoreLabel::ARRAY.map(|label| {
                    device.create_buffer(&wgpu::BufferDescriptor {
                        size: (label.struct_size() * (alloc.get_store_count(label) + 1))
                            as wgpu::BufferAddress,
                        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                        label: Some(&format!("Buffer of {}", label.name())),
                    })
                });

                let entries = StoreLabel::ARRAY.map(|label| wgpu::BindGroupEntry {
                    binding: label.bind(),
                    resource: buffers[label as usize].as_entire_binding(),
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Store bind group"),
                    entries: &entries,
                    layout: &layout,
                });

                Self {
                    bind_group,
                    buffers,
                }
            })
            .collect::<Vec<_>>();

        info!(
            "Succesfully created {} store bindings",
            StoreLabel::COUNT * allocs.len()
        );

        (bindings, layout)
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }
    pub fn views<'a>(
        &'a self,
        queue: &'a wgpu::Queue,
    ) -> [Option<wgpu::QueueWriteBufferView<'a>>; StoreLabel::COUNT] {
        self.buffers.each_ref().map(|buf| {
            wgpu::BufferSize::try_from(buf.size())
                .ok()
                .and_then(|size| queue.write_buffer_with(buf, 0, size))
        })
    }
}
