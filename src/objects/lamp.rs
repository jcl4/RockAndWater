use super::VertexAttribute;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LampVertex {
    pub position: [f32; 3],
}

impl VertexAttribute for LampVertex {
    fn description<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<LampVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float3,
            }],
        }
    }
}

pub struct Lamp {}
