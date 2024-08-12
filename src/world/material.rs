
use glam::Vec3A;

use crate::render_registry::{alloc::{BuffersAllocPosition, Position}, pipelines::PipelineLabel, vertex::UniformTriangleVertex};

use super::{color::Color, shape::TriShape, variator::{UpdateCtx, Variator}, world::World};

fn placer_func<'a, T, U: bytemuck::NoUninit+bytemuck::AnyBitPattern>(array: &'a mut [u32], mut f: impl FnMut(T)->U+'a) -> impl FnMut(T)+'a {
    let mut it = bytemuck::cast_slice_mut(array).iter_mut();
    move |v| {
        *it.next().unwrap() = f(v)
    }
}

pub trait Material {
    fn pipeline(&self) -> PipelineLabel;
    fn size_u32(&self) -> (usize, usize);
    fn alloc(&self, alloc: &mut BuffersAllocPosition) -> Position {
        let pipe = self.pipeline();
        let (vertex_size, index_size) = self.size_u32();
        alloc.alloc(pipe, vertex_size, index_size)
    }
    fn put(&self, ctx: UpdateCtx, world: &World, vertex: &mut [u32], index_offset: u32, index: &mut [u32]);
}

pub struct UniformTri<S, C> {
    pub shape: S,
    pub color: C,
}
impl<S: TriShape, C: Variator<Item=Color>> Material for UniformTri<S, C> {
    fn pipeline(&self) -> PipelineLabel {
        PipelineLabel::UniformTriangle
    }
    fn size_u32(&self) -> (usize, usize) {
        let (vertex_count, index_count) = self.shape.size();
        (vertex_count * UniformTriangleVertex::SIZE_U32, index_count)
    }
    fn put(&self, ctx: UpdateCtx, world: &World, vertex: &mut [u32], index_offset: u32, index: &mut [u32]) {
        let col = self.color.update(ctx, world).as_u32();
        let add_vertex = placer_func(vertex, |pos: Vec3A| UniformTriangleVertex(pos.into(), col));
        let add_index = placer_func(index, |idx| idx+index_offset);
        self.shape.put(add_vertex, add_index, ctx, world);
    }
}
