// use std::ops::Range;
// use crate::materials::materials::Material;
// use crate::materials::pipelines::PipelineLabel;
// use crate::materials::shape::Shape;
// use crate::utils::macros::array_key;

// struct ShapeInfo {
//     vertex_count: usize,
//     include_uv: bool,
//     vertex_size: usize,
//     index_size: usize,
//     ty: ShapeType,
// }
// struct MaterialInfo {
//     color_channel_count: usize,
//     require_uv: bool,
//     ty: MaterialType,
// }
// array_key!(
//     enum MaterialType {
//         Uniform,
//     }
// );
// array_key!(
//     enum ShapeType {
//         Triangle,
//     }
// );


// pub fn dispatch(current: &mut BuffersAllocPosition, material: &Material, shape: &Shape) -> Option<Position> {
//     let material_info = match material {
//         Material::Uniform(_) => MaterialInfo {
//             color_channel_count: 1,
//             require_uv: false,
//             ty: MaterialType::OneCol,
//         },
//     };
//     let shape_info = match shape {
//         Shape::Shape2d() => ShapeInfo { 
//             vertex_count: 3, // TODO
//             include_uv: false, 
//             vertex_size: 12,
//             index_size: 3,
//             ty: ShapeType::Triangle,
//         },
//     };
//     let pipe: PipelineLabel = match (material_info.ty, shape_info.ty) {
//         (MaterialType::Uniform, ShapeType::Triangle) => PipelineLabel::UniformTriangle,
//         _ => { return None; }
//     };
//     let vertex_size = shape_info.vertex_size;
//     Some(current.alloc(pipe, vertex_size, index_size))
// }
