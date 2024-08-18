use glam::{Vec2, Vec4};
use winit::dpi::{PhysicalPosition, PhysicalSize, Pixel};

// macro_rules! inter_casts {
//     (
//         $(
//             $start: ty
//             $($inter: ty),*
//             $(,)?
//         );*
//         $(;)?
//     ) => {
        
//     };
// }

/// like Into
pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<T: Pixel> CastInto<Vec2> for PhysicalPosition<T> {
    fn cast_into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        t.into()
    }
}
impl<T: Pixel> CastInto<Vec2> for PhysicalSize<T> {
    fn cast_into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        t.into()
    }
}
impl<T: Pixel> CastInto<PhysicalPosition<T>> for Vec2 {
    fn cast_into(self) -> PhysicalPosition<T> {
        let t: (f32, f32) = self.into();
        t.into()
    }
}
impl<T: Pixel> CastInto<PhysicalSize<T>> for Vec2 {
    fn cast_into(self) -> PhysicalSize<T> {
        let t: (f32, f32) = self.into();
        t.into()
    }
}

pub type CompressedVec = [u8; 4];
pub fn compress_vec4_u(vec: Vec4) -> CompressedVec {
    (vec.clamp(Vec4::ZERO, Vec4::ONE) * 255.)
        .to_array()
        .map(|f| f as u8)
}
pub fn compress_vec4_i(vec: Vec4) -> CompressedVec {
    (vec.clamp(Vec4::NEG_ONE, Vec4::ONE) * 128.)
        .to_array()
        .map(|f| (f as i8).to_ne_bytes()[0])
}
