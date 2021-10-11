use glium::{IndexBuffer, VertexBuffer};

use super::cam::camera::Camera;
use super::cam::projection_ortho::ProjectionOrtho;
use super::game_object::{GameObject, Transform};
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
    camera: Camera,
}

impl System {
    pub fn new() -> Self {
        let projection = ProjectionOrtho::new(0.0, 1080.0, 0.0, 1440.0, -1.0, 1.0);
        Self {
            game_objects: Vec::new(),
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            textures: Vec::new(),
            camera: Camera::new([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], projection),
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
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}
