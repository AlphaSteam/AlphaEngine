use std::collections::HashMap;

use owning_ref::MutexGuardRef;
use owning_ref::MutexGuardRefMut;
use std::sync::Arc;
use std::sync::Mutex;
use super::game_object::GameObject;

#[derive(Clone,Default)]
pub struct GameObjects{

        game_objects: Arc<Mutex<HashMap<String, Box<dyn GameObject>>>>,

}
impl GameObjects{
    pub fn new(game_objects:  HashMap<String, Box<dyn GameObject>>)->Self{
GameObjects{game_objects: Arc::new(Mutex::new(game_objects))}
    }
    
    pub fn game_objects(&self) -> MutexGuardRef<HashMap<String, Box<dyn GameObject>>>{
        MutexGuardRef::new(self.game_objects.lock().unwrap())
    }
    pub fn game_objects_script(&mut self) -> MutexGuardRef<HashMap<String, Box<dyn GameObject>>>{
        MutexGuardRef::new(self.game_objects.lock().unwrap())
    }
    pub fn game_objects_mut(&mut self) -> MutexGuardRefMut<HashMap<String, Box<dyn GameObject>>> {
        MutexGuardRefMut::new(self.game_objects.lock().unwrap())
    }

    pub fn get_game_object_mut(
        &mut self,
        game_object_id: String,
    ) -> MutexGuardRefMut<HashMap<String, Box<dyn GameObject>>, Box<dyn GameObject>> {
         self.game_objects_mut().map_mut(|mg| mg.get_mut(&game_object_id).unwrap())
        
       
    }
    pub fn add_game_object(&mut self, game_object_id: String, game_object: Box<dyn GameObject>) {
        let game_objects = self.game_objects_mut();
        game_objects.map(|mg| mg.entry(game_object_id).or_insert(game_object));
        
        //entry(game_object_id).or_insert(game_object);
    }
   
   
   
    
  /*   pub fn remove_game_object(&mut self, game_object_id: String) {
        let game_objects = self.game_objects_mut();
        *game_objects.map(
            
            |mg| {mg.remove(&game_object_id).unwrap();} 
    
    
    
    );

        //game_objects.map(|mg| &mg.remove_entry(&game_object_id).unwrap(); );
        
    } */

}