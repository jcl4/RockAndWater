pub struct Mesh<T> {
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
}

impl<T> Mesh<T> {
    pub fn new(vertices: Vec<T>, indices: Vec<u16>) -> Mesh<T> {
        Mesh { vertices, indices }
    }
}

pub trait VertexAttribute {
    fn description<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}
