use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::graphics::menubar::MenuBar;

struct ExampleRepaintSingal { }

impl  epi::backend::RepaintSignal for ExampleRepaintSingal {
    fn request_repaint(&self) {

    }
}


#[derive(Default)]
pub struct AppWindow {
    window: Option<Window>,
    counter: i32,
    title: String,
    width: u32,
    height: u32,
    menu_bar: MenuBar,
}

impl AppWindow {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        AppWindow {
            window: None,
            counter: 0,
            title,
            width,
            height,
            menu_bar: MenuBar::new(),
        }
    }
    pub fn start(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        println!("Seconds");
        let _ = event_loop.run_app(self);
    }
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(PhysicalSize::new(self.width, self.height));

        println!("First");

        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        let window_clone = self.window.clone().unwrap();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(&window_clone).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            }, None
        )).unwrap();

        let size = window_clone.inner_size();

        // let (device, queue) = pollster::block_on(adapter.request_device(
        //     &wgpu::DeviceDescriptor {
        //         features: wgpu::Features::default(),
        //         limits: wgpu::Limits::default(),
        //         label: None,
        //     },
        //     None,
        // ))
        //     .unwrap();

    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        // Handle window event.
        match event {
            WindowEvent::RedrawRequested => {
                let mut ctx = egui::Context::default();
                // let raw_input: egui::RawInput =  gather_input();
                // let output = ctx.?

                // // Game loop:
                // loop {
                //     let raw_input: egui::RawInput = gather_input();
                //
                //     let full_output = ctx. run(raw_input, |ctx| {
                //         egui::CentralPanel::default().show(&ctx, |ui| {
                //             ui. label("Hello world!");
                //             if ui. button("Click me").clicked() {
                //                 // take some action here
                //             }
                //         });
                //     });
                //     handle_platform_output(full_output. platform_output);
                //     let clipped_primitives = ctx. tessellate(full_output. shapes, full_output. pixels_per_point);
                //     paint(full_output. textures_delta, clipped_primitives);
                // }
                //

                // egui::TopBottomPanel::top("menubar").show(&self.window, |ui| {
                //     self.menu_bar.update(ui);
                // });

                self.window.as_mut().unwrap().request_redraw();
            },
            _ => {}
        }
    }
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        // Handle Device event.

    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
            self.counter += 1;
        }
    }
}
