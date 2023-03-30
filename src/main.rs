use glium::glutin::event;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{implement_vertex, Display, Surface, VertexBuffer};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn euc2polar(euclidean: [f32; 2]) -> [f32; 2] {
    let x = euclidean[0];
    let y = euclidean[1];
    let radius = (x * x + y * y).sqrt();
    let angle = (y / x).atan();
    return [radius, angle];
}

fn polar2euc(polar: [f32; 2]) -> [f32; 2] {
    let radius = polar[0];
    let angle = polar[1];
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    return [x, y];
}

fn rotate(v: Vertex, angles: f32) -> Vertex {
    let mut polar = euc2polar(v.position);
    polar[1] += angles;
    let new_vertex = Vertex {
        position: polar2euc(polar),
    };
    return new_vertex;
}

fn main() {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

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
    let mut vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let mut vertex2 = Vertex {
        position: [0.0, -0.5],
    };
    let mut vertex3 = Vertex {
        position: [0.5, 0.25],
    };

    let mut frame_number: f32 = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        frame_number += 1.0;
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        // draw triangle
        vertex1 = rotate(vertex1, 0.001);
        vertex2 = rotate(vertex2, 0.001);
        vertex3 = rotate(vertex3, 0.001);
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        match ev {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
