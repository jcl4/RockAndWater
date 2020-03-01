use crate::{Config, Result};
use log::info;
use std::{path::Path, time::Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::input::InputState;
use crate::model::{Mesh, Model, Transform, Vertex};
use crate::renderer::Renderer;

fn create_triangle(renderer: &Renderer) -> Result<Model> {
    let vert_path = Path::new("./resources/shaders/shader.vert");
    let frag_path = Path::new("./resources/shaders/shader.frag");
    let pipeline = renderer.create_pipeline(vert_path, frag_path)?;

    let transform = Transform::default();

    let vertices = [
        Vertex {
            position: [0.0, -0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let indices = [0, 1, 2];

    let mesh = Mesh::new(vertices.to_vec(), indices.to_vec());

    let model = Model::new(transform, mesh, pipeline, &renderer);

    Ok(model)
}

pub struct App {
    window: Window,
    event_loop: EventLoop<()>,
    input_state: InputState,
    renderer: Renderer,
    model: Model,
}

impl App {
    pub fn new(config: Config) -> Result<App> {
        let input_state = InputState::new();

        let init_start = Instant::now();

        let (window, event_loop) = {
            let width = config.window.width;
            let height = config.window.height;

            let title = config.application.name;

            let event_loop = EventLoop::new();
            let size: PhysicalSize<u32> = PhysicalSize::from((width, height));

            let window = WindowBuilder::new()
                .with_inner_size(size)
                .with_title(title)
                .build(&event_loop)?;
            (window, event_loop)
        };
        info!("Window and Event Loop Created");

        let renderer = Renderer::new(&window);
        let triangle = create_triangle(&renderer)?;

        info!(
            "Initialization time: {:#?} sec",
            Instant::now().duration_since(init_start).as_secs_f32()
        );

        Ok(App {
            window,
            event_loop,
            input_state,
            renderer,
            model: triangle,
        })
    }

    pub fn run(self) {
        info!("Event Loop Starting");
        let mut input_state = self.input_state;
        let window = self.window;
        let mut renderer = self.renderer;
        let model = self.model;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    if input_state.is_key_pressed(VirtualKeyCode::Escape) {
                        info!("Escape Key Pressed.");
                        *control_flow = ControlFlow::Exit;
                    }
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    renderer.render(&model);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    ..
                } => renderer.resize(physical_size),
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                    ..
                } => renderer.resize(*new_inner_size),
                Event::LoopDestroyed => {
                    info!("Loop Destroyed");
                }
                Event::DeviceEvent { event, .. } => {
                    input_state.update(&event);
                }
                // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
                // dispatched any events. This is ideal for games and similar applications.
                _ => *control_flow = ControlFlow::Poll,
            }
        });
    }
}
