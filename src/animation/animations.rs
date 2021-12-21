use std::collections::HashMap;

use crate::rendering::{texture::Texture, vertex::Vertex, mesh::Mesh};

#[derive(Clone, Debug)]
pub struct Animations {
    textures: HashMap<String, Texture>,
    current_animation: String,
    last_animation: String,
    current_index: usize,
    frame_length: f32,
    time_passed: f32,
}

impl Animations {
    pub fn new(textures: HashMap<String, Texture>) -> Self {
        Animations {
            current_animation: String::new(),
            last_animation: String::new(),
            current_index: 0,
            frame_length:0.16,
            time_passed: 0.0,
            textures
        }
    }
    pub fn current_animation(&self)->&String{

        &self.current_animation
    }
    pub fn last_animation(&self)->&String{

        &self.last_animation
    }

    pub fn current_animation_mut(&mut self)->&mut String{

        &mut self.current_animation
    }
    pub fn last_animation_mut(&mut self)->&mut String{

        &mut self.last_animation
    }
    pub fn run(&mut self,delta_time: f32){
        let max_index = self.get_current_animation().meta_data().sprites.len();
        self.time_passed += delta_time;
        if self.time_passed >= self.frame_length {
                self.current_index += 1;
                self.time_passed = 0.0;
                if self.current_index >= max_index {
                    self.current_index = 0;
                }
            }
    }
  

    pub fn get_current_animation(&self) -> &Texture {
        self.textures.get(&self.current_animation).unwrap()
    }

    pub fn set_current_animation(&mut self, name: String) {
        self.last_animation = self.current_animation.clone();
        self.current_animation = name;
        self.current_index = 0;
    }
    pub fn set_last_animation(&mut self, name: String) {
        self.last_animation = name;

    }
    pub fn textures(&self)-> &HashMap<String, Texture>{
        &self.textures
    }
    pub fn textures_mut(&mut self)->&mut HashMap<String, Texture>{
        &mut self.textures
    }
    pub fn current_index(&self)->usize{
        self.current_index
    }
    pub fn get_current_frame(&self)->Vec<Vertex>{
        Mesh::get_vertex_buffer_animated(self.get_current_animation().meta_data(),self.current_index)
    }
    pub fn frame_length(&self)->f32{
        self.frame_length
    }
    pub fn set_frame_length(&mut self, frame_length:f32){
        self.frame_length = frame_length;
    }
}