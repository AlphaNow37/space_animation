use crate::app::App;
use std::time::Instant;
use tracing::{info, info_span};
use winit::event::WindowEvent;

pub struct Clock {
    startup: Instant,
    last_render: Instant,
    min_delta: f32,
}
impl Clock {
    pub fn new() -> Self {
        Self {
            startup: Instant::now(),
            last_render: Instant::now(),
            min_delta: 1. / 60.,
        }
    }
    pub fn should_update(&self) -> bool {
        self.last_render.elapsed().as_secs_f32() > self.min_delta
    }
}

pub fn check_update(app: &mut App, event: &WindowEvent) {
    if !matches!(event, WindowEvent::RedrawRequested) {
        return;
    }
    let _span = info_span!("update").entered();
    let now = Instant::now();
    let delta = now - app.clock.last_render;
    let time = (now - app.clock.startup).as_secs_f32();
    if app.key_binds.window_debug.show_fps.is_active() {
        info!(
            "delta={}ms, fps={}, time={}",
            delta.as_millis(),
            1. / delta.as_secs_f32(),
            time
        );
    }
    app.clock.last_render = now;

    if let Some(holder) = &mut app.window {
        app.camera
            .update(delta.as_secs_f32(), &holder.window, &app.key_binds);
        holder.registry.base_bindings.set_time(&app.queue, time); //, app.clock.loop_time);
        app.scene
            .update(&mut holder.registry, &app.queue, time, &app.camera);
    }
}
