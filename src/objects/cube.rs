use super::{Mesh, Object, Transform, VertexAttribute};
use crate::na;
use crate::{Renderer, Result};
use std::{mem, path::Path};

pub struct Cube {
    mesh: Mesh<CubeVertex>,
    pub pipeline: wgpu::RenderPipeline,
    transform: Transform,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Cube {
    pub fn new(renderer: &Renderer) -> Result<Cube> {
        let vert_path = Path::new("./resources/shaders/phong.vert");
        let frag_path = Path::new("./resources/shaders/phong.frag");

        let mesh = create_cube_mesh();

        let position = na::Vector3::<f32>::zeros();
        let scale = 1.0;
        let orientation = na::UnitQuaternion::identity();

        let transform = Transform::from_parts(position.into(), orientation, scale);
        let pipeline = renderer.create_pipeline(vert_path, frag_path, CubeVertex::description())?;

        let vertex_buffer = renderer
            .device
            .create_buffer_mapped(mesh.vertices.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&mesh.vertices);

        let index_buffer = renderer
            .device
            .create_buffer_mapped(mesh.indices.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&mesh.indices);

        let num_indices = mesh.indices.len() as u32;
        println!("Number of Indices: {:?}", num_indices);

        Ok(Cube {
            mesh,
            transform,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        })
    }
}

impl Object for Cube {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffers(0, &[(&self.vertex_buffer, 0)]);
        render_pass.set_index_buffer(&self.index_buffer, 0);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn update(&mut self) {}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CubeVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl VertexAttribute for CubeVertex {
    fn description<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<CubeVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

pub fn create_cube_mesh() -> Mesh<CubeVertex> {
    let front_face = vec![
        CubeVertex {
            position: [0.5, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        CubeVertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        CubeVertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
        CubeVertex {
            position: [-0.5, 0.5, 0.0],
            color: [1.0, 0.0, 1.0],
        },
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh::new(front_face, indices)
}
