use crate::rendering::vertex::Vertex;
use glium::{Display, VertexBuffer};

/**
    Struct that represents an object mesh.

*/
//#[derive(Copy, Clone)]
pub struct Mesh<'a> {
    vertices: &'a [Vertex],
    indices: glium::index::NoIndices, //&'a [i32],
}

impl<'a> Mesh<'a> {
    /**
    Initializer of a Mesh.
    */
    pub fn new( vertices: &'a [Vertex], indices: glium::index::NoIndices) -> Self {
        let mesh = Mesh {
            vertices,
            indices,
        };
        mesh
    }

    /**
    Inmutable access to the vertices of the mesh.
    */
    pub fn vertices(&self) -> &'a [Vertex] {
        &self.vertices
    }

    /**
    Mutable access to the vertices of the mesh.
    */
    pub fn vertices_mut(&mut self) -> &'a [Vertex] {
        self.vertices
    }

    /**
    Inmutable access to the indices of the mesh.
    */
    pub fn indices(&self) -> &glium::index::NoIndices {
        &self.indices
    }

    /**
    Mutable access to the indices of the mesh.
    */
    pub fn indices_mut(&mut self) -> &glium::index::NoIndices {
        &mut self.indices
    }

    /**
    Create a rectangle mesh with provided with and height.
    */
    pub fn create_rectangle() -> Self {
        let vertices = &[
            Vertex {
                position: [-1.0, 1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
            },
        ];
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let mesh = Mesh {
            vertices,
            indices,
        };
        mesh
    }
}
