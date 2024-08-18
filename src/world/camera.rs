use glam::{Affine3A, Mat4};

use super::{rotation::Angle, variator::Variator};


#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub pos: Affine3A,
    pub fov: Angle,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Affine3A::IDENTITY,
            fov: Angle::from_deg(120.),
        }
    }
}
impl Camera {
    pub fn matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_infinite_lh(self.fov.rad(), aspect_ratio, 0.1)
        * Mat4::from(self.pos.inverse())
    }
}

pub struct GetManualCamera;
impl Variator for GetManualCamera {
    type Item = Camera;
    fn update(&self, _ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        world.settings.cam_settings
    }
}
pub struct TrackCamera<P, F>(pub P, pub F);
impl<P: Variator<Item=Affine3A>, F: Variator<Item=Angle>> Variator for TrackCamera<P, F> {
    type Item = Camera;
    fn update(&self, ctx: super::variator::UpdateCtx, world: &super::world::World) -> Self::Item {
        Camera {
            pos: self.0.update(ctx, world),
            fov: self.1.update(ctx, world),
        }
    }
}
