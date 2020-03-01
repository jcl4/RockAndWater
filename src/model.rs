mod mesh;
mod transform;

pub use mesh::Mesh;
pub use mesh::Vertex;
pub use transform::Transform;

use crate::renderer::{Pipeline, Renderer};

pub struct Model {
    transform: Transform,
    pub mesh: Mesh,
    pub pipeline: Pipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}

impl Model {
    pub fn new(transform: Transform, mesh: Mesh, pipeline: Pipeline, renderer: &Renderer) -> Model {
        let vertex_buffer = create_vertex_buffer(&mesh.vertices, &renderer.device);
        let num_vertices = mesh.vertices.len() as u32;
        Model {
            transform,
            mesh,
            pipeline,
            vertex_buffer,
            num_vertices,
        }
    }
}

fn create_vertex_buffer(vertices: &Vec<Vertex>, device: &wgpu::Device) -> wgpu::Buffer {
    device
        .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(vertices)
}
