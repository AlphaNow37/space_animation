// use glam::Vec4;

// #[derive(Debug)]
// pub enum Material {
//     Uniform(Vec4),
// }

use std::default;

use glam::Vec3;

use super::{alloc::{BuffersAllocPosition, Position}, color::Color, pipelines::PipelineLabel, vertex::UniformTriangleVertex};

fn placer_func<'a, T, U: bytemuck::NoUninit+bytemuck::AnyBitPattern>(array: &'a mut [u32], mut f: impl FnMut(T)->U+'a) -> impl FnMut(T)+'a {
    let mut it = bytemuck::cast_slice_mut(array).iter_mut();
    move |v| {
        *it.next().unwrap() = f(v)
    }
}

#[derive(Clone, Debug)]
pub enum FlatShape {
    Triangle([Vec3; 3]),
}
impl FlatShape {
    fn size(&self) -> (usize, usize) {
        match self {
            Self::Triangle(..) => (3, 3),
        }
    }
    fn put(&self, mut add_vertex: impl FnMut(Vec3), mut add_index: impl FnMut(u32)) {
        match self {
            Self::Triangle(pts) => {
                for pt in pts {add_vertex(*pt)};
                for idx in [0, 1, 2] {add_index(idx)};
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum Material {
    #[default]
    None,
    UniformFlat {
        col: Color,
        shape: FlatShape,
    }
}
impl Material {
    pub fn pipeline(&self) -> PipelineLabel {
        match self {
            Self::None => PipelineLabel::UniformTriangle, // do not matter
            Self::UniformFlat { .. } => PipelineLabel::UniformTriangle,
        }
    }
    fn size_u32(&self) -> (usize, usize) {
        match self {
            Self::None => (0, 0),
            Self::UniformFlat { col, shape } => {
                let (vertex_count, index_count) = shape.size();
                (vertex_count * UniformTriangleVertex::SIZE_U32, index_count)
            }
        }
    }
    pub fn alloc(&self, alloc: &mut BuffersAllocPosition) -> Position {
        let pipe = self.pipeline();
        let (vertex_size, index_size) = self.size_u32();
        alloc.alloc(pipe, vertex_size, index_size)
    }
    pub fn put(&self, time: f32, mut vertex: &mut [u32], index_offset: u32, index: &mut [u32]) {
        match self {
            Self::None => (),
            Self::UniformFlat { col, shape } => {
                let add_vertex = placer_func(vertex, |pos| UniformTriangleVertex(pos, col.as_u32()));
                let add_index = placer_func(index, |idx| idx+index_offset);
                shape.put(add_vertex, add_index);
            }
        }
    }
}
