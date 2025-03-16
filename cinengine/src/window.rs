// #[derive(Default)]
// struct App {
//     window: Option<Window>,
// }

// impl ApplicationHandler for App {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
//     }

//     fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("The close button was pressed; stopping");
//                 event_loop.exit();
//             },
//             WindowEvent::RedrawRequested => {
//                 // Redraw the application.
//                 //
//                 // It's preferable for applications that do not render continuously to render in
//                 // this event rather than in AboutToWait, since rendering in here allows
//                 // the program to gracefully handle redraws requested by the OS.

//                 // Draw.

//                 // Queue a RedrawRequested event.
//                 //
//                 // You only need to call this if you've determined that you need to redraw in
//                 // applications which do not always need to. Applications that redraw continuously
//                 // can render here instead.
//                 self.window.as_ref().unwrap().request_redraw();
//             }
//             _ => (),
//         }
//     }
// }

// fn main() {
//     let event_loop = EventLoop::new().unwrap();

//     event_loop.set_control_flow(ControlFlow::Poll);

//     event_loop.set_control_flow(ControlFlow::Wait);

//     let mut app = App::default();
//     event_loop.run_app(&mut app);
// }