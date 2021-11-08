use std::collections::HashMap;

use super::cam::camera::Camera;
use super::cam::projection_ortho::ProjectionOrtho;
use super::fullscreen::Fullscreen;
use super::game_object::GameObject;
pub use crate::game::Game;
use crate::text::{font::Font, render_text::Text};
pub use crate::window::Window;
use crate::{rendering::vertex::Vertex, shaders::Shader};
use glium::{Display, IndexBuffer, VertexBuffer};
use glutin::dpi::PhysicalSize;
/**
Struct that hosts the engine functions


*/
#[derive(Debug)]
pub struct System {
    game_objects: HashMap<String, GameObject>,
    vertex_buffers: HashMap<String, VertexBuffer<Vertex>>,
    index_buffers: HashMap<String, IndexBuffer<u32>>,
    textures: HashMap<String, glium::texture::SrgbTexture2d>,
    camera: Camera,
    display: Display,
    current_shader: Shader,
    fonts: HashMap<String, Font>,
    text: Vec<Text>,
    text_buffers: Vec<(VertexBuffer<Vertex>, char)>,
    frame_time_target_nanos: u64,
}

impl System {
    pub fn new(display: Display) -> Self {
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
            game_objects: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            textures: HashMap::new(),
            camera: Camera::new([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], projection),
            display: display.clone(),
            current_shader: Shader::Basic,
            fonts: HashMap::new(),
            text: Vec::new(),
            text_buffers: Vec::new(),
            frame_time_target_nanos: (1_000_000_000 / 60),
        }
    }
    pub fn game_objects(&self) -> &HashMap<String, GameObject> {
        &self.game_objects
    }

    pub fn game_objects_mut(&mut self) -> &mut HashMap<String, GameObject> {
        &mut self.game_objects
    }

    pub fn get_game_object_mut(&mut self, game_object_id: String) -> Option<&mut GameObject> {
        let entry = self.game_objects.entry(game_object_id);
        match entry {
            std::collections::hash_map::Entry::Occupied(object) => Some(object.into_mut()),
            std::collections::hash_map::Entry::Vacant(_) => None,
        }
    }
    pub fn add_game_object(
        &mut self,
        game_object_id: String,
        game_object: GameObject,
    ) -> GameObject {
        let game_objects = self.game_objects_mut();
        game_objects
            .entry(game_object_id)
            .or_insert(game_object.clone());
        game_object
    }
    pub fn remove_game_object(&mut self, game_object_id: String) -> Option<GameObject> {
        let game_objects = self.game_objects_mut();
        game_objects.remove(&game_object_id)
    }

    pub fn vertex_buffers(&self) -> &HashMap<String, VertexBuffer<Vertex>> {
        &self.vertex_buffers
    }

    pub fn vertex_buffers_mut(&mut self) -> &mut HashMap<String, VertexBuffer<Vertex>> {
        &mut self.vertex_buffers
    }
    pub fn add_vertex_buffer(
        &mut self,
        game_object_id: String,
        vertex_buffer: VertexBuffer<Vertex>,
    ) {
        self.vertex_buffers_mut()
            .entry(game_object_id)
            .or_insert(vertex_buffer);
    }

    pub fn index_buffers(&self) -> &HashMap<String, IndexBuffer<u32>> {
        &self.index_buffers
    }

    pub fn index_buffers_mut(&mut self) -> &mut HashMap<String, IndexBuffer<u32>> {
        &mut self.index_buffers
    }

    pub fn add_index_buffer(&mut self, game_object_id: String, index_buffer: IndexBuffer<u32>) {
        self.index_buffers_mut()
            .entry(game_object_id)
            .or_insert(index_buffer);
    }

    pub fn textures(&self) -> &HashMap<String, glium::texture::SrgbTexture2d> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut HashMap<String, glium::texture::SrgbTexture2d> {
        &mut self.textures
    }

    pub fn add_texture(&mut self, game_object_id: String, texture: glium::texture::SrgbTexture2d) {
        self.textures_mut().entry(game_object_id).or_insert(texture);
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

    pub fn get_window_resolution(&mut self) -> [f32; 2] {
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
    pub fn set_framerate_target(&mut self, framerate: f32) {
        let seconds = 1.0 / framerate;
        let nano_seconds = seconds * 1_000_000_000.0;
        self.frame_time_target_nanos = nano_seconds as u64
    }
    pub fn frame_time_target_nanos(&self) -> u64 {
        self.frame_time_target_nanos
    }
    pub fn framerate_target(&self) -> f32 {
        1.0 / (self.frame_time_target_nanos as f32 / 1_000_000_000.0)
    }
}
