use glam::{Affine3A, Mat4};

use super::variator::Variator;


#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    pub pos: Affine3A,
}
impl Camera {
    pub fn as_array(&self) -> [f32; 16] {
        let matrix = Mat4::from(self.pos);
        matrix.to_cols_array()
    }
}

pub struct GetManualCamera;
impl Variator for GetManualCamera {
    type Item = Camera;
    fn update(&self, _ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        world.settings.cam_settings
    }
}