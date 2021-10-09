use glium::{IndexBuffer, VertexBuffer};

use super::game_object::GameObject;
pub use crate::game::Game;
pub use crate::rendering::renderer::Renderer;
use crate::rendering::vertex::Vertex;
pub use crate::window::Window;
/**
Struct that hosts the engine functions


*/
#[derive(Debug)]
pub struct System {
    game_objects: Vec<GameObject>,
    vertex_buffers: Vec<VertexBuffer<Vertex>>,
    index_buffers: Vec<IndexBuffer<u32>>,
    textures: Vec<glium::texture::SrgbTexture2d>,
}

impl System {
    pub fn new() -> Self {
        Self {
            game_objects: Vec::new(),
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            textures: Vec::new(),
        }
    }
    pub fn game_objects(&self) -> &Vec<GameObject> {
        &self.game_objects
    }

    pub fn game_objects_mut(&mut self) -> &mut Vec<GameObject> {
        &mut self.game_objects
    }

    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects_mut().push(game_object)
    }

    pub fn vertex_buffers(&self) -> &Vec<VertexBuffer<Vertex>> {
        &self.vertex_buffers
    }

    pub fn vertex_buffers_mut(&mut self) -> &mut Vec<VertexBuffer<Vertex>> {
        &mut self.vertex_buffers
    }
    pub fn add_vertex_buffer(&mut self, vertex_buffer: VertexBuffer<Vertex>) {
        self.vertex_buffers_mut().push(vertex_buffer)
    }

    pub fn index_buffers(&self) -> &Vec<IndexBuffer<u32>> {
        &self.index_buffers
    }

    pub fn index_buffers_mut(&mut self) -> &mut Vec<IndexBuffer<u32>> {
        &mut self.index_buffers
    }

    pub fn add_index_buffer(&mut self, index_buffer: IndexBuffer<u32>) {
        self.index_buffers_mut().push(index_buffer)
    }

    pub fn textures(&self) -> &Vec<glium::texture::SrgbTexture2d> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut Vec<glium::texture::SrgbTexture2d> {
        &mut self.textures
    }

    pub fn add_texture(&mut self, texture: glium::texture::SrgbTexture2d) {
        self.textures_mut().push(texture)
    }
}
