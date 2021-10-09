use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
