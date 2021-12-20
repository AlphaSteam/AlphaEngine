use std::collections::HashMap;

use super::cam::camera::Camera;
use super::cam::projection_ortho::ProjectionOrtho;
use super::fullscreen::Fullscreen;
use super::game_objects::GameObjects;
pub use crate::game::Game;
use crate::game_objects::game_object::GameObject;
use crate::game_objects::implementations::generic_game_object::GenericGameObject;
use crate::text::{render_text::Text};
pub use crate::window::Window;
use crate::{audio::audio_engine::AudioEngine, net::Net};
use crate::{rendering::vertex::Vertex, shaders::Shader};
use glium::{ IndexBuffer, VertexBuffer};
use glutin::dpi::PhysicalSize;
use laminar::Config;
use rg3d_sound::{
    buffer::SoundBufferResource,
    context::SoundContext,
    pool::Handle,
    source::{generic::GenericSourceBuilder, SoundSource},
};
use rhai::plugin::*;
use rhai::{Engine, Scope};
use glium::backend::glutin::Display;

/**
Struct that hosts the engine functions

*/

pub struct System {
    game_objects: GameObjects,
    vertex_buffers: HashMap<String, VertexBuffer<Vertex>>,
    index_buffers: HashMap<String, IndexBuffer<u32>>,
    textures: HashMap<String, glium::texture::SrgbTexture2d>,
    camera: Camera,
    display: Display,
    current_shader: Shader,
    //fonts: HashMap<String, Font>,
    text: Vec<Text>,
    text_buffers: Vec<(VertexBuffer<Vertex>, char)>,
    frame_time_target_nanos: u64,
    audio_engine: AudioEngine,
    net: Option<Net>,
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
            game_objects: GameObjects::new(HashMap::new()),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            textures: HashMap::new(),
            camera: Camera::new([0.0, 0.0, 10.0], [0.0, 0.0, 1.0], projection),
            display: display.clone(),
            current_shader: Shader::Basic,
            //fonts: HashMap::new(),
            text: Vec::new(),
            text_buffers: Vec::new(),
            frame_time_target_nanos: (1_000_000_000 / 60),
            audio_engine: AudioEngine::new(),
            net: None,
        }
    }
    
    pub fn game_objects(&self) -> &GameObjects {
        &self.game_objects
    }
    pub fn game_objects_mut(&mut self) -> &mut GameObjects{
        &mut self.game_objects
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

    /* pub fn add_font(&mut self, font_name: &str, font_path: &str) {
        let font = Font::new(font_path, &self.display);
        self.fonts.insert(font_name.to_string(), font);
    } */
    /* pub fn fonts(&self) -> &HashMap<String, Font> {
        &self.fonts
    } */
  /*   pub fn fonts_mut(&mut self) -> &mut HashMap<String, Font> {
        &mut self.fonts
    } */
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
        color: [f32; 3],
    ) {
        let text = Text::new(text, font, position, scale, color);
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
    pub fn audio_engine(&self)-> &AudioEngine{
        &self.audio_engine
    }
    pub fn audio_engine_mut(&mut self)-> &mut AudioEngine{
        &mut self.audio_engine
    }
    pub fn create_sound_context(&self) -> SoundContext {
        self.audio_engine.create_sound_context(&self.camera())
    }
    pub fn sound_contexts(&self) -> &HashMap<String, SoundContext> {
        &self.audio_engine.sound_contexts()
    }

    pub fn sound_contexts_mut(&mut self) -> &mut HashMap<String, SoundContext> {
        self.audio_engine.sound_contexts_mut()
    }

    pub fn get_sound_context_mut(&mut self, sound_context_id: String) -> Option<&mut SoundContext> {
        let entry = self
            .audio_engine
            .sound_contexts_mut()
            .entry(sound_context_id);
        match entry {
            std::collections::hash_map::Entry::Occupied(object) => Some(object.into_mut()),
            std::collections::hash_map::Entry::Vacant(_) => None,
        }
    }
    pub fn get_sound_context(&self, sound_context_id: String) -> Option<&SoundContext> {
        let entry = self
            .audio_engine
            .sound_contexts()
            .get_key_value(&sound_context_id);
        match entry {
            Some((_, sound_context)) => Some(sound_context),
            None => todo!(),
        }
    }
    pub fn add_sound_context(
        &mut self,
        sound_context_id: String,
        sound_context: SoundContext,
    ) -> SoundContext {
        let sound_contexts = self.sound_contexts_mut();
        sound_contexts
            .entry(sound_context_id)
            .or_insert(sound_context.clone());
        self.audio_engine
            .sound_engine()
            .lock()
            .unwrap()
            .add_context(sound_context.clone());
        sound_context
    }
    pub fn remove_sound_context(&mut self, sound_context_id: String) -> Option<SoundContext> {
        let sound_contexts = self.sound_contexts_mut();
        sound_contexts.remove(&sound_context_id)
    }

    pub fn add_sound_buffer_from_file(
        &mut self,
        sound_buffer_id: String,
        sound_path: String,
        stream: bool,
    ) -> SoundBufferResource {
        self.audio_engine
            .add_sound_buffer_from_file(sound_buffer_id, sound_path, stream)
    }
    pub fn sound_sources(&self) -> &HashMap<String, Handle<SoundSource>> {
        &self.audio_engine.sound_sources()
    }

    pub fn sound_sources_mut(&mut self) -> &mut HashMap<String, Handle<SoundSource>> {
        self.audio_engine.sound_sources_mut()
    }
    pub fn create_sound_source_from_generic(
        &self,
        generic_source_builder: GenericSourceBuilder,
        spatial: bool,
    ) -> SoundSource {
        self.audio_engine
            .create_sound_source_from_generic(generic_source_builder, spatial)
    }
    pub fn add_sound_source(
        &mut self,
        sound_source_id: String,
        sound_source: Handle<SoundSource>,
    ) -> Handle<SoundSource> {
        self.audio_engine
            .add_sound_source(sound_source_id, sound_source)
    }
    pub fn get_sound_source(&self, sound_source_id: String) -> Option<&Handle<SoundSource>> {
        self.audio_engine.get_sound_source(sound_source_id)
    }
    pub fn get_sound_source_mut(
        &mut self,
        sound_source_id: String,
    ) -> Option<&mut Handle<SoundSource>> {
        self.audio_engine.get_sound_source_mut(sound_source_id)
    }
    pub fn remove_sound_source(&mut self, sound_source_id: String) -> Option<Handle<SoundSource>> {
        self.audio_engine.remove_sound_source(sound_source_id)
    }
    #[allow(unused_must_use)]
    pub fn add_source_to_context(
        &mut self,
        sound_context_id: String,
        source: SoundSource,
    ) -> Handle<SoundSource> {
        self.audio_engine
            .add_source_to_context(sound_context_id, source)
    }
    pub fn connect_to_network(&mut self, server_address: String, client_address: String) {
        self.net = Some(Net::connect(
            server_address.parse().unwrap(),
            client_address.parse().unwrap(),
        ))
    }
    pub fn connect_to_network_with_config(
        &mut self,
        server_address: String,
        client_address: String,
        config: Config,
    ) {
        self.net = Some(Net::connect_with_config(
            server_address.parse().unwrap(),
            client_address.parse().unwrap(),
            config,
        ))
    }
    pub fn send_packet(&self, payload: Vec<u8>) {
        self.net.as_ref().unwrap().send_packet(payload);
    }
    pub fn test_packets(&self) {
        self.net.as_ref().unwrap().test_packets();
    }
    
    pub fn run_script(&mut self, script: String){
        let mut engine = Engine::new_raw();
        type GameObjectsHash =HashMap<String, Box<dyn GameObject>>;
        engine
        .register_type_with_name::<Box<dyn GameObject>>("GameObject")
        .register_fn("game_object_from_sprite", GenericGameObject::game_object_from_sprite_script);

        engine
        .register_type_with_name::<GameObjectsHash>("GameObjects");
        

        let scripts = exported_module!(engine_scripts);
        engine.register_global_module(scripts.into());
        

    let mut scope = Scope::new();
    let game_objects = self.game_objects.game_objects_mut().clone();

    
    scope.push("game_objects", game_objects.clone());
    
    let _err = engine.run_with_scope(&mut scope, &script).unwrap();

    let new_game_objects: HashMap<String, Box<dyn GameObject>>  = scope.get_value("game_objects").unwrap();
    *self.game_objects.game_objects_mut() = new_game_objects;

    }
    
}
#[export_module]
pub mod engine_scripts {
    use std::{collections::HashMap};
    use rhai::Dynamic;
    
    pub fn add_game_object(game_objects: &mut HashMap<String, Box<dyn GameObject>>,game_object_id: String, game_object_dynamic: Dynamic) {
        let game_object: GenericGameObject = game_object_dynamic.cast(); 
        game_objects.entry(game_object_id).or_insert(Box::new(game_object));
    }
    pub fn len(game_objects: HashMap<String, Box<dyn GameObject>>) {
        println!("{}",game_objects.len());
    
        }
   
}
