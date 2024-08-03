use std::sync::Arc;
use tracing::{info_span, warn};
use winit::dpi::{LogicalSize, Size};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use crate::app::App;
use crate::app::scene::recreate_scene;
use crate::materials::registry::PipelinesRegistry;

pub struct SurfaceHolder {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub registry: PipelinesRegistry,
}

fn cfg_window(win: &Window) {
    win.set_min_inner_size(Some(Size::Logical(LogicalSize::new(700., 500.))));
}

impl SurfaceHolder {
    pub fn new(app: &App, event_loop: &ActiveEventLoop) -> Self {
        let _span = info_span!("create_win_surf");

        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        cfg_window(&window);

        let surface = app.instance.create_surface(window.clone()).unwrap();
        let caps = surface.get_capabilities(&app.adapter);
        let format = caps.formats.iter().find(|f| f.is_srgb()).cloned()
            .unwrap_or_else(|| {
                warn!("Can't find any supported sRGB format");
                caps.formats[0]
            });
        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            width: size.width,
            height: size.height,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            present_mode: caps.present_modes[0],
        };

        let mut registry = PipelinesRegistry::new(&app.device, &surface_config);
        app.queue.write_buffer(&registry.pipes[0].vertex_buffer, 0, bytemuck::cast_slice(&[1.0f32; 20]));
        recreate_scene(&mut registry);

        Self {
            window: window.clone(),
            // simple_pipeline: SimplePipeline::new(&app.device, &surface_config),
            registry,
            surface_config,
            surface,
        }
    }
}
