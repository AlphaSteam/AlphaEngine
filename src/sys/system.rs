use std::collections::HashMap;

use super::cam::camera::Camera;
use super::cam::projection_ortho::ProjectionOrtho;
use super::fullscreen::Fullscreen;
use super::game_object::GameObject;
pub use crate::game::Game;
pub use crate::window::Window;
use crate::{
    event::event_manager::EventManager,
    text::{font::Font, render_text::Text},
};
use crate::{rendering::vertex::Vertex, shaders::Shader};
use glium::{Display, IndexBuffer, VertexBuffer};
use glutin::dpi::PhysicalSize;
/**
Struct that hosts the engine functions


*/
pub struct System {
    game_objects: Vec<GameObject>,
    vertex_buffers: Vec<VertexBuffer<Vertex>>,
    index_buffers: Vec<IndexBuffer<u32>>,
    textures: Vec<glium::texture::SrgbTexture2d>,
    camera: Camera,
    display: Display,
    current_shader: Shader,
    fonts: HashMap<String, Font>,
    text: Vec<Text>,
    text_buffers: Vec<(VertexBuffer<Vertex>, char)>,
    event_manager: EventManager,
}

impl System {
    pub fn new(display: Display, event_manager: EventManager) -> Self {
        let window_resolution = display.gl_window().window().inner_size();
        let projection = ProjectionOrtho::new(
            0.0,
            window_resolution.width as f32,
            0.0,
            window_resolution.height as f32,
            -10.0,
            10.0,
        );
        Self {
            game_objects: Vec::new(),
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            textures: Vec::new(),
            camera: Camera::new([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], projection),
            display,
            current_shader: Shader::Basic,
            fonts: HashMap::new(),
            text: Vec::new(),
            text_buffers: Vec::new(),
            event_manager,
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

    pub fn get_window_resolution(&self) -> [f32; 2] {
        let size = self.display.gl_window().window().inner_size();
        [size.width as f32, size.height as f32]
    }
    pub fn set_window_resolution(&self, resolution: [u32; 2]) {
        let size = PhysicalSize {
            width: resolution[0],
            height: resolution[1],
        };
        self.display.gl_window().window().set_maximized(false);
        self.display.gl_window().window().set_inner_size(size);
    }
    pub fn set_window_fullscreen(&self, fullscreen: Fullscreen) {
        self.display
            .gl_window()
            .window()
            .set_fullscreen(fullscreen.value(self.display.gl_window().window().current_monitor()));
    }
    pub fn set_window_maximized(&self, maximized: bool) {
        self.display.gl_window().window().set_maximized(maximized);
    }
    pub fn set_current_shader(&mut self, shader: Shader) {
        self.current_shader = shader
    }
    pub fn current_shader(&self) -> &Shader {
        &self.current_shader
    }

    pub fn add_font(&mut self, font_name: &str, font_path: &str) {
        let font = Font::new(font_path, &self.display);
        self.fonts.insert(font_name.to_string(), font);
    }
    pub fn fonts(&self) -> &HashMap<String, Font> {
        &self.fonts
    }
    pub fn fonts_mut(&mut self) -> &mut HashMap<String, Font> {
        &mut self.fonts
    }
    pub fn text(&self) -> &Vec<Text> {
        &self.text
    }
    pub fn text_mut(&mut self) -> &mut Vec<Text> {
        &mut self.text
    }
    pub fn add_text_buffer(&mut self, vertex_buffer: VertexBuffer<Vertex>, texture: char) {
        self.text_buffers.push((vertex_buffer, texture))
    }
    pub fn text_buffers(&self) -> &Vec<(VertexBuffer<Vertex>, char)> {
        &self.text_buffers
    }
    pub fn render_text(
        &mut self,
        text: String,
        font: String,
        position: [f32; 2],
        scale: [f32; 2],
        rotation: f32,
        color: [f32; 3],
    ) {
        let text = Text::new(text, font, position, scale, rotation, color);
        self.text.push(text);
    }

    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }
    pub fn event_manager_mut(&mut self) -> &mut EventManager {
        &mut self.event_manager
    }
}
