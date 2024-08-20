use winit::dpi::{PhysicalPosition, PhysicalSize, Pixel};
use crate::math::{Vec2, Vec4};

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

impl<T: Pixel> Into<Vec2> for PhysicalPosition<T> {
    fn into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        Vec2::new(t.0, t.1)
    }
}
impl<T: Pixel> Into<Vec2> for PhysicalSize<T> {
    fn into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        Vec2::new(t.0, t.1)
    }
}
impl<T: Pixel> Into<PhysicalPosition<T>> for Vec2 {
    fn into(self) -> PhysicalPosition<T> {
        (self.x(), self.y()).into()
    }
}
impl<T: Pixel> Into<PhysicalSize<T>> for Vec2 {
    fn into(self) -> PhysicalSize<T> {
        (self.x(), self.y()).into()
    }
}

pub type CompressedVec = [u8; 4];
pub fn compress_vec4_u(vec: Vec4) -> CompressedVec {
    (vec.clamp(Vec4::ZERO, Vec4::ONE) * 255.)
        .to_array()
        .map(|f| f as u8)
}
pub fn compress_vec4_i(vec: Vec4) -> CompressedVec {
    (vec.clamp(-Vec4::ONE, Vec4::ONE) * 128.)
        .to_array()
        .map(|f| (f as i8).to_ne_bytes()[0])
}
