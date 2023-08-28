#[macro_use]
extern crate glium;

use std::process;
use std::time::Instant;

use glium::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::Surface;

mod config;
mod shader;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    // Read the config file for the program.
    let conf = match config::read_config() {
        Ok(config) => config,
        Err(error) => {
            println!("{error}");
            process::exit(1);
        }
    };

    // 1. The **winit::EventsLoop** for handling events.
    let events_loop = EventLoop::new();

    // 2. Parameters for building the Window.
    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            conf.window.width,
            conf.window.height,
        ))
        .with_title(&conf.window.title);

    // 3. Parameters for building the OpenGL context.
    let context_builder = glium::glutin::ContextBuilder::new();

    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = match glium::Display::new(window_builder, context_builder, &events_loop) {
        Ok(display) => display,
        Err(error) => {
            println!("Could not create glium display: {error}");
            process::exit(2);
        }
    };

    // 5. Create some shape to draw.
    let shape = vec![
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
    ];
    let vertex_buffer = match glium::VertexBuffer::new(&display, &shape) {
        Ok(buffer) => buffer,
        Err(error) => {
            println!("Could not create vertex buffer for rendering: {error}");
            process::exit(3);
        }
    };
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // 6. Load Shaders.
    let program = match shader::load_shader(&display, &conf.shader) {
        Ok(program) => program,
        Err(error) => {
            println!("{error}");
            process::exit(4);
        }
    };

    // 7. Finally run the event loop.
    let loop_start_time = Instant::now();
    events_loop.run(move |ev, _, control_flow| {
        let start_time = Instant::now();

        // Poll events
        match ev {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => display.gl_window().resize(size),
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            },

            Event::RedrawRequested(_) => {
                // Draw frames
                let mut target = display.draw();
                target.clear_color(0.0, 1.0, 0.0, 1.0);
                match target.draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &uniform! {
                        iResolution: [
                            display.gl_window().window().inner_size().width as f32,
                            display.gl_window().window().inner_size().height as f32,
                            0.0
                        ],
                        iTime: start_time.duration_since(loop_start_time).as_secs_f32(),
                    },
                    &Default::default(),
                ) {
                    Ok(_) => (),
                    Err(error) => println!("Failed to draw a frame: {error}"),
                };
                match target.finish() {
                    Ok(_) => (),
                    Err(error) => println!("Failed to swap a frame buffer: {error}"),
                };
            }

            _ => (),
        };

        // Handle FPS limit
        match *control_flow {
            ControlFlow::Exit | ControlFlow::ExitWithCode(_) => (),
            _ => {
                display.gl_window().window().request_redraw();

                let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;
                let wait_millis = match 1000 / conf.fps >= elapsed_time {
                    true => 1000 / conf.fps - elapsed_time,
                    false => 0,
                };
                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                *control_flow = ControlFlow::WaitUntil(new_inst);
            }
        };
    })
}
