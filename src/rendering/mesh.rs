
use sheep::SerializedSpriteSheet;

use crate::{rendering::vertex::Vertex, helpers::math::normalize_number};

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
                position: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ];
        let indices = vec![0, 1, 2, 3, 0, 2];
        let mesh = Mesh { vertices, indices };
        mesh
    }
   
    pub fn get_vertex_buffer_animated(sprite_sheet: &SerializedSpriteSheet, sprite_index: usize) -> Vec<Vertex>{
        let max_width = sprite_sheet.texture_width;
        let max_height = sprite_sheet.texture_height;
        let sprites = sprite_sheet.sprites.clone();
        let sprite = &sprites[sprite_index];
        let min_x = normalize_number(sprite.x,0.0,max_width);
        let max_x  = normalize_number(sprite.x + sprite.width ,0.0,max_width);

        let min_y = normalize_number(sprite.y,0.0,max_height);
        let max_y = normalize_number(sprite.y + sprite.height,0.0,max_height);



        let left_up = [min_x, min_y];
        let right_up = [max_x, min_y];
        let left_down = [min_x,max_y];
        let right_down = [max_x, max_y];

        
        vec![
            Vertex {
                position: [0.0, 1.0, 0.0],
                tex_coords: left_down,
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: left_up,
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                tex_coords: right_up,
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: right_down,
            },
        ]

    }
    pub fn create_rectangle_animated(sprite_sheet: &SerializedSpriteSheet, sprite_index: usize) -> Self{
        let vertices = Mesh::get_vertex_buffer_animated(sprite_sheet, sprite_index);
        let indices = vec![0, 1, 2, 3, 0, 2];
        let mesh = Mesh { vertices, indices};
        mesh
    }
}
