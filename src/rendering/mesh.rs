use crate::rendering::vertex::Vertex;

/**
    Struct that represents an object mesh.

*/
#[derive(Clone, Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    /**
    Initializer of a Mesh.
    */
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mesh = Mesh { vertices, indices };
        mesh
    }

    /**
    Inmutable access to the vertices of the mesh.
    */
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    /**
    Mutable access to the vertices of the mesh.
    */
    pub fn vertices_mut(&mut self) -> &Vec<Vertex> {
        &self.vertices
    }

    /**
    Inmutable access to the indices of the mesh.
    */
    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    /**
    Mutable access to the indices of the mesh.
    */
    pub fn indices_mut(&mut self) -> &Vec<u32> {
        &mut self.indices
    }

    /**
    Create a rectangle mesh with provided with and height.
    */
    pub fn create_rectangle() -> Self {
        let vertices = vec![
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 1, 3, 2];
        let mesh = Mesh { vertices, indices };
        mesh
    }
}
