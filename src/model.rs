mod mesh;
mod transform;

pub use mesh::Mesh;
pub use mesh::Vertex;
pub use transform::Transform;

use crate::renderer::{Pipeline, Renderer, Texture};

pub struct Model {
    transform: Transform,
    pub mesh: Mesh,
    pub pipeline: Pipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,

    pub texture: Texture,
}

impl Model {
    pub fn new(
        transform: Transform,
        mesh: Mesh,
        pipeline: Pipeline,
        texture: Texture,
        renderer: &Renderer,
    ) -> Model {
        let vertex_buffer = create_vertex_buffer(&mesh.vertices, &renderer.device);
        let index_buffer = create_index_buffer(&mesh.indices, &renderer.device);
        let num_indices = mesh.indices.len() as u32;
        Model {
            transform,
            mesh,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture,
        }
    }
}

fn create_vertex_buffer(vertices: &Vec<Vertex>, device: &wgpu::Device) -> wgpu::Buffer {
    device
        .create_buffer_mapped(vertices.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(vertices)
}

fn create_index_buffer(indices: &Vec<u16>, device: &wgpu::Device) -> wgpu::Buffer {
    device
        .create_buffer_mapped(indices.len(), wgpu::BufferUsage::INDEX)
        .fill_from_slice(indices)
}
