use std::collections::HashMap;

use owning_ref::MutexGuardRef;
use owning_ref::MutexGuardRefMut;
use owning_ref::OwningRef;
use rhai::Dynamic;
use crate::sys::game_object::GenericGameObject;
use std::sync::Arc;
use std::sync::Mutex;
use super::game_object::GameObject;

#[derive(Clone, Debug)]
pub struct GameObjects{

        game_objects: Arc<Mutex<HashMap<String, Box<dyn GameObject>>>>,

}
impl GameObjects{
    pub fn new(game_objects:  HashMap<String, Box<dyn GameObject>>)->Self{
GameObjects{game_objects: Arc::new(Mutex::new(game_objects))}
    }
    //Arc<Mutex<HashMap<String, Box<dyn GameObject>>>>
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
   
    pub fn add_game_object_script(&mut self, game_object_id: String, game_object_dynamic: Dynamic) {
        let game_objects = self.game_objects_mut();
        println!("Is variant: {}",game_object_dynamic.is_variant());
        // println!("Type name: {}",game_object_dynamic.type_name());

        let game_object: GenericGameObject = game_object_dynamic.cast(); 
        game_objects.map(|mg| mg.entry(game_object_id).or_insert(Box::new(game_object)));

        //println!("Game objects inside script: {}",game_objects.clone().len());

    }
   
    
  /*   pub fn remove_game_object(&mut self, game_object_id: String) {
        let game_objects = self.game_objects_mut();
        *game_objects.map(
            
            |mg| {mg.remove(&game_object_id).unwrap();} 
    
    
    
    );

        //game_objects.map(|mg| &mg.remove_entry(&game_object_id).unwrap(); );
        
    } */

}