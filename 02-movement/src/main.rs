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

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Position shader
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float t;
        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    // Color(?) shader
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    // Wrap all the shader stuff up together
    let program = glium::Program::from_source(
        &display, 
        vertex_shader_src, 
        fragment_shader_src, 
        None
    ).unwrap();


    let v1 = Vertex { position: [-0.5, -0.5] };
    let v2 = Vertex { position: [ 0.0, 0.5 ] };
    let v3 = Vertex { position: [ 0.5, -0.25 ] };
    let shape = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let mut t: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {
       
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
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            }
            _ => return,
        }

        // update 't'
        t += 0.0005;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniform! {t: t},  // &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();
    });
}
