use crate::utils::macros::array_key;

macro_rules! source {
    (
        $name: literal
    ) => {
        {
            wgpu::include_wgsl!(concat!("../shaders/", $name, ".wgsl"))
        }
    };
}

array_key!(
    pub enum ShaderFile {
        Simple,
    }
);

impl ShaderFile {
    pub fn source(&self) -> wgpu::ShaderModuleDescriptor {
        match self {
            Self::Simple => source!("simple"),
        }
    }
}

pub struct Shaders {
    shaders: [wgpu::ShaderModule; ShaderFile::COUNT]
}
impl Shaders {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            shaders: ShaderFile::ARRAY.map(|v| device.create_shader_module(v.source())),
        }
    }
    pub fn get(&self, file: ShaderFile) -> &wgpu::ShaderModule {
        &self.shaders[file as usize]
    }
}
