use crate::math::Angle;
use crate::math::{Mat4, Transform};
use crate::world::variators::variator::Variator;
use crate::world::world::Worlds;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub pos: Transform,
    pub fov: Angle,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Transform::ID,
            fov: Angle::from_deg(90.),
        }
    }
}
impl Camera {
    pub fn matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::new_perspective_infinite_lh(self.fov, aspect_ratio, 0.1)
            * self.pos.inverse().to_mat4()
    }
}

pub struct GetManualCamera;
impl Variator for GetManualCamera {
    type Item = Camera;
    fn update(&self, worlds: &Worlds) -> Self::Item {
        worlds.settings.cam_settings
    }
}

pub struct TrackCamera<P, F>(pub P, pub F);
impl<P: Variator<Item = Transform>, F: Variator<Item = Angle>> Variator for TrackCamera<P, F> {
    type Item = Camera;
    fn update(&self, worlds: &Worlds) -> Self::Item {
        Camera {
            pos: self.0.update(worlds),
            fov: self.1.update(worlds),
        }
    }
}
