use bytemuck::{Pod, Zeroable};
use glam::Vec3;

macro_rules! impl_vertex {
    (
        $ty: ident:
        $size: literal,
        $(
            $n: literal => $val: ident
        ),*
        $(,)?
    ) => {
        impl $ty {
            pub const ATTRS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
                $(
                    $n => $val,
                )*
            ];
            pub const SIZE: usize = $size * 4;
            // pub const SIZE_U32: usize = $size;
            #[allow(dead_code)]
            const CHECK: () = const { assert!(std::mem::size_of::<Self>() == Self::SIZE) };
        }
    };
}

#[derive(Pod, Clone, Copy, Zeroable)]
#[repr(C)]
pub struct UniformTriangleVertex(pub Vec3, pub u32);
impl_vertex!(UniformTriangleVertex: 4, 0 => Float32x3, 1 => Unorm8x4);
