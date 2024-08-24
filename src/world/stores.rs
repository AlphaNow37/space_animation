use crate::render_registry::alloc::BufferAllocator;
use crate::utils::array_key;
use crate::world::world::World;

array_key!(
    pub enum StoreLabel {
        Transforms,
        F32,
        Vec2,
        Vec3,
        Vec4,
        Color,
        Color2,
        Poly4x4,
    }
);
impl StoreLabel {
    pub fn struct_size(self) -> usize {
        16 * match self {
            Self::F32 | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Color => 1,
            Self::Color2 => 2,
            Self::Poly4x4 => 16,
            Self::Transforms => 4,
        }
    }
    pub fn bind(self) -> u32 {
        match self {
            Self::Transforms => 0,
            Self::F32 => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
            Self::Color => 5,
            Self::Color2 => 6,
            Self::Poly4x4 => 7,
        }
    }
    pub fn write(self, buf: &mut [u32], world: &mut World) {
        match self {
            Self::Transforms => world.store_transform(buf),
            Self::F32 => world.store_f32(buf),
            Self::Vec2 => world.store_vec2(buf),
            Self::Vec3 => world.store_vec3(buf),
            Self::Vec4 => world.store_vec4(buf),
            Self::Color => world.store_color(buf),
            Self::Color2 => world.store_color2(buf),
            Self::Poly4x4 => world.store_poly4x4(buf),
        }
    }
    pub fn alloc(self, world: &World, alloc: &mut BufferAllocator) {
        let len = match self {
            Self::Transforms => world.len_transform(),
            Self::F32 => world.len_f32(),
            Self::Vec2 => world.len_vec2(),
            Self::Vec3 => world.len_vec3(),
            Self::Vec4 => world.len_vec4(),
            Self::Color => world.len_color(),
            Self::Color2 => world.len_color2(),
            Self::Poly4x4 => world.len_poly4x4(),
        };
        alloc.alloc_store(self, len);
    }
    pub fn stage(self) -> wgpu::ShaderStages {
        match self {
            Self::Transforms | Self::F32 | Self::Vec2 | Self::Vec3 | Self::Vec4 | Self::Poly4x4 => wgpu::ShaderStages::VERTEX,
            Self::Color | Self::Color2 => wgpu::ShaderStages::FRAGMENT,
        }
    }
}
