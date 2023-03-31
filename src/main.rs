use glium::glutin::event;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{implement_vertex, Display, Surface, VertexBuffer};

use std::{thread, time};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn euc2polar(euclidean: [f32; 2]) -> [f32; 2] {
    let x = euclidean[1];
    let y = euclidean[0];
    let radius = (x * x + y * y).sqrt();
    let angle = (y / x).atan();
    return [radius, angle];
}

fn polar2euc(polar: [f32; 2]) -> [f32; 2] {
    let radius = polar[0];
    let angle = polar[1];
    let x = radius * angle.sin();
    let y = radius * angle.cos();
    return [x, y];
}

// vertex1: 0.55901736 -0.25020933 -> -0.5590169 -0.24923159
// center: 0 -0.25
// corrected_pos: 0.55901736 -0.00020933 -> -0.5590169 -0.24923159

fn rotate(v: Vertex, center: [f32; 2], angles: f32) -> Vertex {
    let corrected_pos = [v.position[0] - center[0], v.position[1] - center[1]];
    let mut polar = euc2polar(corrected_pos);
    polar[1] += angles;

    let mut new_euc = polar2euc(polar);
    new_euc = [new_euc[0] + center[0], new_euc[1] + center[1]];

    Vertex { position: new_euc }
}

fn euc_dist(p1: [f32; 2], p2: [f32; 2]) -> f32 {
    return ((p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1])).sqrt();
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

    let center = [
        (vertex3.position[0] + vertex2.position[0] + vertex1.position[0]) / 3.0,
        (vertex3.position[1] + vertex2.position[1] + vertex1.position[1]) / 3.0,
    ];
    println!("center: {} {}", center[0], center[1]);

    // let mut frame_number: f32 = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        // frame_number += 1.0;
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        thread::sleep(time::Duration::from_millis(1));

        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        // draw triangle
        let new_vertex1 = rotate(vertex1, center, 0.001);
        let new_vertex2 = rotate(vertex2, center, 0.001);
        let new_vertex3 = rotate(vertex3, center, 0.001);
        if euc_dist(new_vertex1.position, vertex1.position) > 0.2 {
            println!(
                "vertex1: {} {} -> {} {}",
                vertex1.position[0],
                vertex1.position[1],
                new_vertex1.position[0],
                new_vertex1.position[1]
            );
        }
        if euc_dist(new_vertex2.position, vertex2.position) > 0.2 {
            println!(
                "vertex2: {} {} -> {} {}",
                vertex2.position[0],
                vertex2.position[1],
                new_vertex2.position[0],
                new_vertex2.position[1]
            );
        }
        if euc_dist(new_vertex3.position, vertex3.position) > 0.2 {
            println!(
                "vertex3: {} {} -> {} {}",
                vertex3.position[0],
                vertex3.position[1],
                new_vertex3.position[0],
                new_vertex3.position[1]
            );
        }

        vertex1 = new_vertex1;
        vertex2 = new_vertex2;
        vertex3 = new_vertex3;

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
