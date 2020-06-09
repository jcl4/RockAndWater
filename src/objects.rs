use crate::na;
mod cube;
mod lamp;
mod mesh;

// pub use lamp::{Lamp, LampVertex};
pub use cube::Cube;
pub use mesh::Mesh;
pub use mesh::VertexAttribute;

type Transform = na::Similarity3<f32>;

pub trait Object {
    fn render(&self, render_pass: &mut wgpu::RenderPass);
    fn update(&mut self);
}
