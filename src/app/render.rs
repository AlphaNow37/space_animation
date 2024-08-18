use tracing::{error, info_span};
use winit::event::WindowEvent;
use crate::app::App;

pub fn check_render(app: &mut App, event: &WindowEvent) {
    if !matches!(event, WindowEvent::RedrawRequested) {
        return;
    }
    let _span = info_span!("render").entered();
    // info!("Rendering");

    let Some(holder) = &app.window else {
        error!("No window while rendering");
        return;
    };

    let output = holder.surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = app.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render encoder")
    });
    holder.registry.render(&mut encoder, &view);
    // render_pass(&mut encoder, &view, &holder.simple_pipeline);
    app.queue.submit([encoder.finish()]);
    output.present();
}
