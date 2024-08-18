use glam::{Affine3A, Mat4};

use super::variator::Variator;


#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Affine3A,
    pub fov: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Affine3A::IDENTITY,
            fov: 120.0f32.to_radians()
        }
    }
}
impl Camera {
    pub fn matrix(&self) -> Mat4 {
        Mat4::perspective_infinite_lh(self.fov, 1.5, 0.1)
        * Mat4::from(self.pos)
    }
    pub fn as_array(&self) -> [f32; 16] {
        self.matrix().to_cols_array()
    }
}

pub struct GetManualCamera;
impl Variator for GetManualCamera {
    type Item = Camera;
    fn update(&self, _ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        world.settings.cam_settings
    }
}