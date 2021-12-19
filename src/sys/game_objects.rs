use std::collections::HashMap;

use crate::game_objects::game_object::GameObject;

#[derive(Clone,Default)]
pub struct GameObjects{

        game_objects: HashMap<String, Box<dyn GameObject>>,

}
impl GameObjects{
    pub fn new(game_objects:  HashMap<String, Box<dyn GameObject>>)->Self{
GameObjects{game_objects}
    }
    
    pub fn game_objects(&self) -> &HashMap<String, Box<dyn GameObject>>{
        &self.game_objects
    }
    pub fn game_objects_script(&mut self) ->&HashMap<String, Box<dyn GameObject>>{
        &self.game_objects
    }
    pub fn game_objects_mut(&mut self) -> &mut HashMap<String, Box<dyn GameObject>> {
        &mut self.game_objects
    }

    pub fn get_game_object_mut(
        &mut self,
        game_object_id: String,
    ) ->&mut Box<dyn GameObject> {
         self.game_objects_mut().get_mut(&game_object_id).unwrap()
        
       
    }
    pub fn add_game_object(&mut self, game_object_id: String, game_object: Box<dyn GameObject>) {
        let game_objects = self.game_objects_mut();
        game_objects.entry(game_object_id).or_insert(game_object);
        
    }
   
   
   
    
    pub fn remove_game_object(&mut self, game_object_id: String) {
        let game_objects = self.game_objects_mut();
        game_objects.remove_entry(&game_object_id).unwrap();
            
    }

}