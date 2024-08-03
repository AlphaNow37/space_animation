// use tracing::{info, info_span};
// use wgpu::include_wgsl;
// use crate::app::App;

// macro_rules! load {
//     ($device: expr, $name: literal) => {
//         {
//             use wgpu::include_wgsl;
//             $device.create_shader_module(include_wgsl!(concat!("../shaders/", $name)))
//         }
//     };
// }
// pub(crate) use load;
// pub struct Shaders {
//     simple: wgpu::ShaderModule,
// }
// impl Shaders {
//     pub fn load(device: &wgpu::Device) -> Self {
//         let _span = info_span!("load_shaders").entered();
//         info!("Loading shaders");
//         let shaders = Self {
//             simple: load!(device, "simple.wgsl"),
//         };
//         info!("Shaders loaded");
//         shaders
//     }
// }
