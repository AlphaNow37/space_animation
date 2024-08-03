use std::ops::Range;
use crate::materials::materials::Material;
use crate::materials::pipelines::PipelineLabel;
use crate::materials::shape::Shape;
use crate::utils::macros::array_key;

array_key!(
    enum MaterialType {
        Uniform,
    }
);
array_key!(
    enum ShapeType {
        Triangle,
    }
);

#[derive(Default)]
pub struct CurrentPosition {
    vertex_index: [(usize, usize); PipelineLabel::COUNT],
}

pub struct Position {
    pipe_label: PipelineLabel,
    vertex_bound: Range<usize>,
    index_bound: Range<usize>,
}

pub fn dispatch(current: &mut CurrentPosition, material: &Material, shape: &Shape) -> Position {
    let material_type = match material {
        Material::Uniform(_) => MaterialType::Uniform,
    };
    let shape_type = match shape {
        Shape::Shape2d() => ShapeType::Triangle,
    };
    let (pipe, vertex_size, index_size): (PipelineLabel, usize, usize) = match (material_type, shape_type) {
        (MaterialType::Uniform, ShapeType::Triangle) => (PipelineLabel::UniformTriangle, 0, 0),
    };
    let (start_vertex, start_index) = &mut current.vertex_index[pipe as usize];
    let pos = Position {
        pipe_label: pipe,
        index_bound: *start_index..*start_index+index_size,
        vertex_bound: *start_vertex..*start_vertex+vertex_size,
    };
    *start_vertex = pos.vertex_bound.end;
    *start_index = pos.index_bound.end;
    pos
}
