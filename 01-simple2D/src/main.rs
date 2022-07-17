extern crate glium;
use glium::{glutin, Surface};
use std::time::{Instant, Duration};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2];
}
implement_vertex!(Vertex, position);

fn get_frame_time() -> Instant {
    return Instant::now() + Duration::from_nanos(16_666_667);
}

fn main() {
    
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(640, 480))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
       
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = get_frame_time();
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return
                },
                _ => return,
            },
            _ => (),
        }
    });
}
