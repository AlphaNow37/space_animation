use std::ops::Mul;

use glam::Vec3;


macro_rules! consts {
    (
        $($name: ident = $r: expr, $g: expr, $b: expr);*
        $(;)?
    ) => {
        impl Color {
            $(
                pub const $name: Self = Self(Vec3::new($r, $g, $b));
            )*
        }
    };
}

pub struct Color(pub Vec3);
impl Color {
    pub fn as_u32(&self) -> u32 {
        u32::from_le_bytes(self.0.extend(1.).to_array().map(|c| c.clamp(0., 1.).mul(255.) as u8))
    }
}
consts!(
    WHITE = 0., 0., 0.;
    DEBUG = 0.8, 0., 0.4;
);
