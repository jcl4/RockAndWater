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
use crate::renderer::{texture::Texture, Renderer};

fn create_model(renderer: &mut Renderer) -> Result<Model> {
    let vert_path = Path::new("./resources/shaders/shader.vert");
    let frag_path = Path::new("./resources/shaders/shader.frag");
    let texture_path = Path::new("./resources/textures/Fabric38_col.jpg");
    let texture = Texture::new(texture_path, &renderer.device, &mut renderer.queue)?;

    let pipeline =
        renderer.create_pipeline(vert_path, frag_path, &texture.diffuse_bind_group_layout)?;

    let transform = Transform::default();

    let vertices = [
        Vertex {
            position: [-0.0868241, -0.49240386, 0.0],
            tex_coords: [0.4131759, 0.99240386],
        }, // A
        Vertex {
            position: [-0.49513406, -0.06958647, 0.0],
            tex_coords: [0.0048659444, 0.56958646],
        }, // B
        Vertex {
            position: [-0.21918549, 0.44939706, 0.0],
            tex_coords: [0.28081453, 0.050602943],
        }, // C
        Vertex {
            position: [0.35966998, 0.3473291, 0.0],
            tex_coords: [0.85967, 0.15267089],
        }, // D
        Vertex {
            position: [0.44147372, -0.2347359, 0.0],
            tex_coords: [0.9414737, 0.7347359],
        }, // E
    ];

    let indices = [0, 1, 4, 1, 2, 4, 2, 3, 4];

    let mesh = Mesh::new(vertices.to_vec(), indices.to_vec());

    let model = Model::new(transform, mesh, pipeline, texture, &renderer);

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

        let mut renderer = Renderer::new(&window);
        let model = create_model(&mut renderer)?;

        info!(
            "Initialization time: {:#?} sec",
            Instant::now().duration_since(init_start).as_secs_f32()
        );

        Ok(App {
            window,
            event_loop,
            input_state,
            renderer,
            model,
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
