use crate::rendering::vertex::Vertex;
use glium::Display;

#[derive(Copy, Clone)]
pub struct Mesh<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [Vertex],
}

impl<'a> Mesh<'a> {
    fn new() -> Self {
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let mesh = Mesh {};
        mesh
    }
}
