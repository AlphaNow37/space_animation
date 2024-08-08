use std::ops::DerefMut;

use glam::Vec3;
use log::info;
use tracing::info_span;
// use crate::app::App;
use crate::{materials::{alloc::{BuffersAllocPosition, Position}, color::Color, materials::{FlatShape, Material}, pipelines::PipelineLabel, registry::PipelinesRegistry}, world::{entity::Entity, world::{EntityRef, World}}};

struct PutGlobalCtx<'a> {
    time: f32,
    views: [&'a mut [u32]; PipelineLabel::COUNT],
}
struct PutCtx {
    ent: EntityRef,
}

pub struct Scene {
    pub final_position: BuffersAllocPosition,
    entities: Vec<Entity>,
    allocs: Vec<Position>,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");
        let mut world = World::new();
        crate::content::build(&mut world);

        let mut alloc = BuffersAllocPosition::new();
        let allocs = world.entities.iter()
            .map(|e| e.material.alloc(&mut alloc))
            .collect();

        Scene {
            final_position: alloc,
            entities: world.entities,
            allocs,
        }
    }
    fn put_ent(&self, ctx: PutCtx, gctx: &mut PutGlobalCtx) {
        let id = ctx.ent.as_usize();
        let ent = &self.entities[id];
        let alloc = &self.allocs[id];
        let vertex = &mut gctx.views[alloc.pipe_label as usize][alloc.vertex_bound.clone()];
        ent.material.put(gctx.time, vertex, 0, &mut [0; 100]);
        for child in &ent.childs {
            self.put_ent(PutCtx {ent: *child}, gctx);
        }
    }
    pub fn update(&self, registry: &mut PipelinesRegistry, queue: &wgpu::Queue, time: f32) {
        let _span = info_span!("update_scene").entered();
        info!("Updating scene");
        let mut views = registry.views(queue);
        let mut gctx = PutGlobalCtx {
            time,
            views: views.each_mut().map(|v| bytemuck::cast_slice_mut(v.deref_mut())),
        };
        let ctx = PutCtx {
            ent: EntityRef::ROOT,
        };
        self.put_ent(ctx, &mut gctx);
    }
}
