// use wgpu::{ShaderModuleDescriptor};
// use crate::materials::pipelines::{pipe_names, PipeNames};
// use crate::materials::shaders::ShaderFile;
// use crate::materials::vertex::VertexKind;
// use crate::utils::macros::array_key;
//
//
//
// impl MaterialType {
//     pub fn names(&self) -> PipeNames {
//         match self {
//             Self::Uniform => pipe_names!("Uniform"),
//         }
//     }
//     pub fn shader_file(&self) -> ShaderFile {
//         match self {
//             Self::Uniform => ShaderFile::Simple,
//         }
//     }
//     pub fn entry_point(&self) -> &'static str {
//         match self {
//             _ => "fs_main",
//         }
//     }
// }
