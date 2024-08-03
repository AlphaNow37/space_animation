// use crate::materials::shaders::ShaderFile;
// use crate::utils::macros::array_key;



// impl VertexKind {
//     pub fn size(&self) -> usize {
//         match self {
//             Self::PosColor => 8,
//         }
//     }
//     pub fn vertex_attributes(&self) -> &'static [wgpu::VertexAttribute] {
//         match self {
//             Self::PosColor => &const {wgpu::vertex_attr_array![
//                 0 => Float32x4,
//                 1 => Float32x4,
//             ]}
//         }
//     }
//     pub fn shader_file(&self) -> ShaderFile {
//         match self {
//             Self::PosColor => ShaderFile::Simple,
//         }
//     }
//     pub fn entry_point(&self) -> &'static str {
//         match self {
//             Self::PosColor => "vs_main",
//         }
//     }
// }

// pub enum Shape2d {
//     Circle(f32),
//     Triangle([Vec2; 3]),
//     Square(Vec2, Vec2),
//     Polyline(Vec<Vec2>),
//     Polygon(Vec<Vec2>),
//     Line(Vec2, Vec2),
// }
// pub struct Shape3d {
//     origin: Affine3A,
//     shape: Shape2d
// }
