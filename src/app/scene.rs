use std::ops::DerefMut;

use glam::Vec3;
use log::info;
use tracing::info_span;
// use crate::app::App;
use crate::{materials::{alloc::BuffersAllocPosition, color::Color, materials::{FlatShape, Material}, pipelines::PipelineLabel, registry::PipelinesRegistry}, world::{entity::Entity, world::{EntityRef, World}}};

struct PutGlobalCtx<'a> {
    time: f32,
    views: [&'a mut [u8]; PipelineLabel::COUNT],
}
struct PutCtx {
    ent: EntityRef,
}

pub struct Scene {
    pub final_position: BuffersAllocPosition,
    entities: Vec<Entity>,
}
impl Scene {
    pub fn new() -> Self {
        let _span = info_span!("new_scene").entered();
        info!("Creating scene");
        let mut world = World::new();
        world.alloc.alloc(PipelineLabel::UniformTriangle, 100, 0);

        Scene {
            final_position: world.alloc,
            entities: world.entities,
        }
    }
    fn put_ent(&self, ctx: PutCtx, gctx: &mut PutGlobalCtx) {

    }
    pub fn update(&self, registry: &mut PipelinesRegistry, queue: &wgpu::Queue, time: f32) {
        let _span = info_span!("update_scene").entered();
        info!("Updating scene");
        let mut views = registry.views(queue);
        let mut gctx = PutGlobalCtx {
            time,
            views: views.each_mut().map(|v| v.deref_mut()),
        };
        let ctx = PutCtx {
            ent: EntityRef::ROOT,
        };
        self.put_ent(ctx, &mut gctx);

        let mat = Material::UniformFlat { col: Color::DEBUG, shape: FlatShape::Triangle([Vec3::ZERO, Vec3::X, Vec3::Y]) };
        mat.put(time, &mut gctx.views[0][..48], 0, &mut [0; 10]);

        queue.submit([]);
    }
}
