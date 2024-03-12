use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};



fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    window.set_title("Wgpu Tutorial by Coded Studios");
    env_logger::init();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
   

}
