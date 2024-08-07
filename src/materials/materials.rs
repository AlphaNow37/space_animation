// use glam::Vec4;

// #[derive(Debug)]
// pub enum Material {
//     Uniform(Vec4),
// }

use glam::Vec3;

use super::{color::Color, pipelines::PipelineLabel, vertex::UniformTriangleVertex};

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

pub enum Material {
    UniformFlat {
        col: Color,
        shape: FlatShape,
    }
}
impl Material {
    pub fn pipeline(&self) -> PipelineLabel {
        match self {
            Self::UniformFlat { .. } => PipelineLabel::UniformTriangle,
        }
    }
    pub fn byte_size(&self) -> (usize, usize) {
        match self {
            Self::UniformFlat { col, shape } => {
                let (vertex_count, index_count) = shape.size();
                (vertex_count * UniformTriangleVertex::SIZE, index_count)
            }
        }
    }
    pub fn put(&self, time: f32, mut vertex: &mut [u8], index_offset: u32, index: &mut [u32]) {
        match self {
            Self::UniformFlat { col, shape } => {
                let vert_array: &mut [UniformTriangleVertex] = bytemuck::cast_slice_mut(vertex);
                let mut v_it = vert_array.iter_mut();
                let add_vertex = move |pos: Vec3| {
                    *v_it.next().unwrap() = UniformTriangleVertex(pos, col.as_u32());
                };
                let mut i_it = index.iter_mut();
                let add_index = |idx: u32| {
                    *i_it.next().unwrap() = idx + index_offset;
                };
                shape.put(add_vertex, add_index);
            }
        }
    }
}
