use glam::Vec2;
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
