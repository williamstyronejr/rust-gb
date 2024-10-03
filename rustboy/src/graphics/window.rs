use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct AppWindow {
    window: Option<Window>,
    counter: i32,
    title: String,
    width: u32,
    height: u32,
}

impl AppWindow {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        AppWindow {
            window: None,
            counter: 0,
            title,
            width,
            height,
        }
    }
    pub fn start(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let _ = event_loop.run_app(self);
    }
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(PhysicalSize::new(self.width, self.height));

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        // Handle window event.
    }
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        // Handle window event.
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
            self.counter += 1;
        }
    }
}
