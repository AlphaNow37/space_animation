use glam::{Mat3A, Quat, Vec2, Vec3A};
use rand::Rng;
use tracing::{info, info_span};
use winit::{dpi::PhysicalPosition, event::WindowEvent, window::Window};

use crate::utils::{CastInto};
use crate::app::keybinds::{KeyBinds, MoveKey};
use crate::world::primitives::camera::Camera;

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
    pub current_cam_idx: isize,
    win_size: Vec2,
    cursor_locked: bool,
}
impl ManualCamera {
    pub fn new() -> Self {
        Self {
            cam: Camera::default(),
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
    fn reset_current(&mut self) {
        self.current_cam_idx = 0;
        info!("Reseted current camera to 0");
    }
    fn rng_current(&mut self) {
        self.current_cam_idx = rand::thread_rng().gen_range(0..100_000_000_000);
        info!("Randomed current camera to {}", self.current_cam_idx);
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
    }
    pub fn update(&mut self, dt: f32, win: &Window, binds: &KeyBinds) {
        let _span = info_span!("camera").entered();
        for mk in MoveKey::ARRAY {
            let mut off = Vec3A::ZERO;
            if binds.camera_moves[mk as usize].is_active() {
                off += move_key_to_dir(mk) * dt
            }
            self.cam.pos.translation += self.cam.pos.transform_vector3a(off);
            if off != Vec3A::ZERO {
                info!("Moved camera to {} ({:+})", self.cam.pos.translation, off);
                // let mat = self.cam.matrix(self.aspect_ratio);
                // info!("(0., 0., 0.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 0.)), mat.project_point3(Vec3::new(0., 0., 0.)));
                // info!("(0., 0., 1.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 1.)), mat.project_point3(Vec3::new(0., 0., 1.)));
            }
        }
        if binds.camera_change.prev_cam.is_active() {
            self.next_current(-1);
        }
        if binds.camera_change.next_cam.is_active() {
            self.next_current(1);
        }
        if binds.camera_change.reset_cam.is_active() {
            self.reset_current();
        }
        if binds.camera_change.rng_cam.is_active() {
            self.rng_current();
        }
        if binds.camera_change.toggle_lock.is_active() {
            self.toggle_lock(win);
        }
        if binds.camera_change.reset_pos.is_active() {
            self.reset();
        }
    }
}
