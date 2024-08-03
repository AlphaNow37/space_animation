use glam::{Affine3A, Vec3};
use winit::keyboard::{KeyCode};

fn key_code_to_dir(code: KeyCode) -> Vec3 {
    use KeyCode::*;
    match code {
        KeyA => Vec3::NEG_X,
        KeyD => Vec3::X,
        KeyS => Vec3::NEG_Y,
        KeyW => Vec3::Y,
        KeyQ => Vec3::NEG_Z,
        KeyE => Vec3::Z,
        _ => Vec3::ZERO,
    }
}

pub struct Camera {
    pos: Affine3A,
    fov: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Affine3A::default(),
            fov: 1.,
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    pub fn as_bytes(&self) -> [u8; 48] {
        let mut fs = [0.; 12];
        self.pos.write_cols_to_slice(&mut fs);
        bytemuck::cast(fs)
    }
    // pub fn on_event(&mut self, event: &WindowEvent) {
    //     match event {
    //         WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                 physical_key: PhysicalKey::Code(c),
    //                 ..
    //             },
    //             ..
    //         } => {
    //             self.pos.translation +=
    //         }
    //     }
    // }
}
