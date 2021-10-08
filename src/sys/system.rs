use glium::Display;

pub use crate::game::Game;
pub use crate::rendering::renderer::Renderer;
pub use crate::window::Window;
use super::game_object::GameObject;
/**
Struct that hosts the engine functions


*/
#[derive(Debug)]
pub struct System {
    game_objects: Vec<GameObject>,

}

impl System {

    pub fn new()->Self{
        Self{
            game_objects: Vec::new(),
        }
    }
    pub fn game_objects(&self) -> &Vec<GameObject> {
        &self.game_objects
    }

    pub fn game_objects_mut(&mut self) -> &mut Vec<GameObject> {
        &mut self.game_objects
    }
  
    pub fn add_game_object(&mut self,game_object:GameObject){
        self.game_objects_mut().push(game_object)
    }
    
}
