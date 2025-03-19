use winit::{
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

pub struct Window {
    pub winit_window: Option<WinitWindow>,
    pub window_id: Option<WindowId>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            winit_window: None,
            window_id: None,
        }
    }

    pub fn default_attributes() -> WindowAttributes {
        WindowAttributes::default()
            .with_title("Cineris")
            .with_inner_size(LogicalSize::new(800.0, 600.0))
    }

    pub fn create(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Self::default_attributes()).unwrap();
        self.window_id = Some(window.id());
        self.winit_window = Some(window);
    }
}