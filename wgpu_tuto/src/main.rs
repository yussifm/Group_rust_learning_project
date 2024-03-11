use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();
      

      // Best for games
    // event_loop.set_control_flow(ControlFlow::Poll);

     // best for apps and use less power/cpu time than control flow
    event_loop.set_control_flow(ControlFlow::Wait);

    window.set_title("Wgpu Tutorial by Coded Studios");
    env_logger::init();

    event_loop.run(move |event, elwt| {
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            },

            // for redrawing
            Event::AboutToWait => {}
            ,
            _ => {}
        }
    });
}
