mod exit;
mod surface_holder;
mod camera;
mod resize;
mod render;
mod update;
mod scene;

use camera::ManualCamera;
use scene::Scene;
use tracing::{info, info_span};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use crate::app::exit::check_exit;
use crate::app::render::check_render;
use crate::app::resize::check_resize;
use crate::app::surface_holder::SurfaceHolder;
use crate::app::update::{check_update, Clock};

// type EventLoop = winit::event_loop::EventLoop<()>;

fn get_adapter(surf: Option<&wgpu::Surface>, inst: &wgpu::Instance) -> wgpu::Adapter {
    let options = surf.map(|s| wgpu::RequestAdapterOptions {
        compatible_surface: Some(&s),
        ..Default::default()
    }).unwrap_or_default();
    pollster::block_on(inst.request_adapter(&options)).unwrap()
}

fn get_device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap()
}

pub struct App {
    pub clock: Clock,
    pub window: Option<SurfaceHolder>,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub scene: Scene,
    pub camera: ManualCamera,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let _span = info_span!("restart").entered();
        let holder = SurfaceHolder::new(self, event_loop);
        if !self.adapter.is_surface_supported(&holder.surface) {
            self.adapter = get_adapter(Some(&holder.surface), &self.instance);
            // usefull ?
            // (self.device, self.queue) = get_device_queue(&self.adapter);
            // self.shaders = Shaders::load(&self.device);
        }
        self.window = Some(holder);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let _span = info_span!("app_update").entered();
        if check_exit(self, &event) {
            event_loop.exit();
            return;
        }
        check_resize(self, &event);
        check_update(self, &event);
        check_render(self, &event);
        if let Some(holder) = &self.window {
            self.camera.on_event(&event, &holder.window);
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(win) = &self.window {
            if self.clock.should_update() {
                win.window.request_redraw()
            }
        };
    }
}
impl App {
    pub fn new() -> Self {
        info!("Creating app");
        let instance = wgpu::Instance::default();
        let adapter = get_adapter(None, &instance);
        let (device, queue) = get_device_queue(&adapter);
        Self {
            clock: Clock::new(),
            window: None,
            adapter,
            instance,
            // shaders: Shaders::load(&device),
            device,
            queue,
            scene: Scene::new(),
            camera: ManualCamera::new(),
        }
    }
    pub fn run(&mut self) {
        info!("Running app");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap()
    }
}
