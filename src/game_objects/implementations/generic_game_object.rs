use rhai::{Array, Map};

use crate::{game_objects::game_object::{BaseGameObjectProperties, GmObj}, sys::system::System, rendering::{mesh::Mesh}, animation::animations::Animations};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GenericGameObject {
    base_properties: BaseGameObjectProperties,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
    action:  fn(&mut System),
}
impl GenericGameObject {
    pub fn new(
        position: [f32; 3],
        size: [f32; 3],
        meshes: HashMap<String, Mesh>,
        animations: Animations,
        z_index: i32,
        should_render: bool,
        ui:bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action: fn(&mut System),
    ) -> Self {
        let base_properties = BaseGameObjectProperties::new(
                                                                                position,
                                                                                size,
                                                                                meshes,
                                                                                animations,
                                                                                z_index,
                                                                                should_render,
                                                                                ui
                                                                            );
        GenericGameObject { base_properties, start, update, stop, action }
    }

    pub fn game_object_from_sprites(
        position: [f32; 3],
        texture_paths: HashMap<String,String>,
        default_texture: String,
        z_index: i32,
        should_render: bool,
        ui:bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action: fn(&mut System)
    ) -> Self {

        let base_properties = BaseGameObjectProperties::game_object_from_sprites(position, texture_paths, default_texture, z_index,should_render,ui);
        GenericGameObject { base_properties, start, update, stop, action }
    }
    pub fn game_object_from_sprites_script(
        position: Array,
        texture_paths: Map,
        default_texture: rhai::ImmutableString,
        z_index: i32,
        should_render:bool,
        ui:bool,
    ) -> Self {
        let position = [position[0].clone_cast::<f64>() as f32,position[1].clone_cast::<f64>() as f32,position[2].clone_cast::<f64>() as f32];
        let mut texture_paths_hash = HashMap::new();
        for (texture_name,texture_path) in texture_paths.iter(){
            let texture_path : String = texture_path.clone().cast();
            let texture_name: String = texture_name.to_string();
            texture_paths_hash.insert(texture_name,texture_path);
        }
        let base_properties = BaseGameObjectProperties::game_object_from_sprites(position, texture_paths_hash, default_texture.to_string(), z_index,should_render,ui);
        GenericGameObject { 
            base_properties,
            start:| _system|{},
            update:| _system|{},
            stop:| _system|{},
            action:| _system|{},
        }
    }
    pub fn base_properties_script(&mut self) -> BaseGameObjectProperties {
        self.base_properties.clone()
    }
}
impl GmObj for GenericGameObject {
    fn base_properties(&self) -> &BaseGameObjectProperties {
        &self.base_properties
    }
    fn base_properties_mut(&mut self) -> &mut BaseGameObjectProperties {
        &mut self.base_properties
    }
    fn start(&mut self) -> fn(&mut System) {
        self.start
    }
    fn update(&mut self) -> fn(&mut System) {
        self.update
    }
    fn stop(&mut self) -> fn(&mut System) {
        self.stop
    }
    fn action(&mut self) -> fn(&mut System){
        self.action
    }
}
