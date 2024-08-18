use glam::Vec3A;
use log::info;
use tracing::info_span;
use winit::{event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

use crate::{utils::macros::array_key, world::camera::Camera};

array_key!(
    enum MoveKey {
        Left,
        Right,
        Up,
        Down,
        Forward,
        Backward,
    }
);

fn key_code_to_key(code: KeyCode) -> Option<MoveKey> {
    use KeyCode::*;
    use MoveKey::*;
    Some(match code {
        KeyA => Left,
        KeyD => Right,
        KeyS => Backward,
        KeyW => Forward,
        KeyQ => Down,
        KeyE => Up,
        _ => {return None;},
    })
}

fn move_key_to_dir(key: MoveKey) -> Vec3A {
    use MoveKey::*;
    match key {
        Left => Vec3A::NEG_X,
        Right => Vec3A::X,
        Backward => Vec3A::NEG_Y,
        Forward => Vec3A::Y,
        Down => Vec3A::NEG_Z,
        Up => Vec3A::Z,
    }
}

pub struct ManualCamera {
    pub cam: Camera,
    key_pressed: [bool; MoveKey::COUNT],
    pub current_cam_idx: isize,
}
impl ManualCamera {
    pub fn new() -> Self {
        Self {
            cam: Camera::default(),
            key_pressed: [false; MoveKey::COUNT],
            current_cam_idx: 0,
        }
    }
    fn reset(&mut self) {
        info!("Reseting camera");
        self.cam = Camera::default();
        self.current_cam_idx = 0;
    }
    fn next_current(&mut self, off: isize) {
        self.current_cam_idx += off;
        info!("Changing current camera to {} ({:+})", self.current_cam_idx, off);
    }
    pub fn on_event(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput {
            event: KeyEvent {
                physical_key: PhysicalKey::Code(code),
                repeat: false,
                state,
                ..
            },
            ..
        } = event {
            if let Some(mk) = key_code_to_key(*code) {
                self.key_pressed[mk as usize] = state.is_pressed()
            }
        }
        if let WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(code), state: ElementState::Pressed, .. }, .. } = event {
            let _span = info_span!("camera").entered();
            match code {
                KeyCode::KeyR => self.reset(),
                KeyCode::KeyL => self.next_current(-1),
                KeyCode::Semicolon => self.next_current(1), // english keyboard is disgusting, why the fuck is semicolon here ?
                _ => {},
            }
        }
    }
    pub fn update(&mut self, dt: f32) {
        for mk in MoveKey::ARRAY {
            if self.key_pressed[mk as usize] {
                self.cam.pos.translation += move_key_to_dir(mk) * dt
            }
        }
    }
}
