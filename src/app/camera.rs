use glam::{Mat3A, Quat, Vec2, Vec3A};
use log::info;
use tracing::info_span;
use winit::{dpi::PhysicalPosition, event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{utils::{macros::array_key, CastInto}, world::camera::Camera};

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
        Backward => Vec3A::NEG_Z,
        Forward => Vec3A::Z,
        Down => Vec3A::NEG_Y,
        Up => Vec3A::Y,
    }
}

pub struct ManualCamera {
    pub cam: Camera,
    key_pressed: [bool; MoveKey::COUNT],
    pub current_cam_idx: isize,
    win_size: Vec2,
    cursor_locked: bool,
}
impl ManualCamera {
    pub fn new() -> Self {
        Self {
            cam: Camera::default(),
            key_pressed: [false; MoveKey::COUNT],
            current_cam_idx: 0,
            win_size: Vec2::ONE,
            cursor_locked: false,
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
    fn toggle_lock(&mut self, win: &Window) {
        self.cursor_locked ^= true;
        if self.cursor_locked {
            info!("Locking cursor");
            win.set_cursor_position(winit::dpi::Position::Physical((self.win_size/2.0).cast_into())).unwrap();
            win.set_cursor_visible(false);
        } else {
            info!("Unlocking cursor");
            win.set_cursor_visible(true);
        }
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.win_size.x / self.win_size.y
    }
    pub fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = new_size.cast_into();
    }
    pub fn on_event(&mut self, event: &WindowEvent, win: &Window) {
        if let WindowEvent::CursorMoved { position, .. } = event {
            if self.cursor_locked {
                let pos: Vec2 = position.cast_into();
                let angle = (pos - self.win_size/2.) / self.win_size.y * self.cam.fov.rad();
                let quat = Quat::from_rotation_x(angle.y) * Quat::from_rotation_y(angle.x);
                self.cam.pos.matrix3 *= Mat3A::from_quat(quat);
                // win.set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
                win.set_cursor_position(PhysicalPosition::new(self.win_size.x/2., self.win_size.y/2.)).unwrap();
            }
        }
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
                self.key_pressed[mk as usize] = state.is_pressed();
            }
        }
        if let WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(code), state: ElementState::Pressed, .. }, .. } = event {
            let _span = info_span!("camera").entered();
            match code {
                KeyCode::KeyR => self.reset(),
                KeyCode::KeyL => self.next_current(-1),
                KeyCode::Semicolon => self.next_current(1), // english keyboard is disgusting, why the fuck is semicolon here ?
                KeyCode::KeyU => self.toggle_lock(win),
                _ => {},
            }
        }
    }
    pub fn update(&mut self, dt: f32) {
        for mk in MoveKey::ARRAY {
            let mut off = Vec3A::ZERO;
            if self.key_pressed[mk as usize] {
                off += move_key_to_dir(mk) * dt
            }
            self.cam.pos.translation += self.cam.pos.transform_vector3a(off);
            if off != Vec3A::ZERO {
                let _span = info_span!("camera").entered();
                info!("Moved camera to {} ({:+})", self.cam.pos.translation, off);
                // let mat = self.cam.matrix(self.aspect_ratio);
                // info!("(0., 0., 0.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 0.)), mat.project_point3(Vec3::new(0., 0., 0.)));
                // info!("(0., 0., 1.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 1.)), mat.project_point3(Vec3::new(0., 0., 1.)));
            }
        }
    }
}
