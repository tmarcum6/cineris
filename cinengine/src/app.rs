use crate::window::Window;
use crate::vulkano::VulkanContext;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

pub struct App {
    window: Window,
    vulkan: VulkanContext,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
            vulkan: VulkanContext::new(),
        }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self); // think i need to add error handling
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.create(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.window_id == Some(id) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = &self.window.winit_window {
                        window.request_redraw();
                    }
                }
                _ => (),
            }
        }
    }
}
