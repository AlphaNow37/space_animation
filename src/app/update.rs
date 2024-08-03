use std::ops::Rem;
use std::time::Instant;
use log::info;
use tracing::info_span;
use winit::event::WindowEvent;
use crate::app::App;

pub struct Clock {
    startup: Instant,
    last_render: Instant,
    min_delta: f32,
    loop_time: f32,
}
impl Clock {
    pub fn new() -> Self {
        Self {
            startup: Instant::now(),
            last_render: Instant::now(),
            min_delta: 1./60.,
            loop_time: 5.,
        }
    }
    pub fn should_update(&self) -> bool {
        self.last_render.elapsed().as_secs_f32() > self.min_delta
    }
}

pub fn check_update(app: &mut App, event: &WindowEvent) {
    if !matches!(event, WindowEvent::RedrawRequested) {return;}
    let _span = info_span!("update").entered();
    let now = Instant::now();
    let delta = now - app.clock.last_render;
    let time = (now - app.clock.startup).as_secs_f32().rem(app.clock.loop_time);
    info!("delta={}ms, fps={}, time={}/{}", delta.as_millis(), 1./delta.as_secs_f32(), time, app.clock.loop_time);
    app.clock.last_render = now;

    if let Some(holder) = &mut app.window {
        holder.registry.set_time(&app.queue, time, app.clock.loop_time)
    }
}
