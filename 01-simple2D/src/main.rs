extern crate glium;
use glium::{glutin, Surface};
use std::time::{Instant, Duration};

fn get_frame_time() -> Instant {
    return Instant::now() + Duration::from_nanos(16_666_667);
}

fn main() {
    #[allow(unused_imports)]
    
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(640, 480))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    glium::implement_vertex!(Vertex, position);

    let vert1 = Vertex { position: [-0.3, -0.5] };
    let vert2 = Vertex { position: [0.1, 0.5] };
    let vert3 = Vertex { position: [0.5, -0.4] };
    let shape = vec![vert1, vert2, vert3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(
        &display, 
        vertex_shader_src, 
        fragment_shader_src, 
        None
    ).unwrap();

    event_loop.run(move |ev, _, control_flow| {
       
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &vertex_buffer, 
            &indices, 
            &program, 
            &glium::uniforms::EmptyUniforms, 
            &Default::default()
        ).unwrap();
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
