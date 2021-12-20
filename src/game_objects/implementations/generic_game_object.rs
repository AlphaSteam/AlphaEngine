use rhai::Array;

use crate::{game_objects::game_object::{BaseGameObjectProperties, GmObj}, sys::system::System, rendering::{mesh::Mesh, texture::Texture}};

#[derive(Clone)]
pub struct GenericGameObject {
    base_properties: BaseGameObjectProperties,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
}
impl GenericGameObject {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture: Texture, z_index: i32, start: fn(&mut System), update: fn(&mut System), stop: fn(&mut System)) -> Self {
        let base_properties = BaseGameObjectProperties::new(position, size, mesh, texture, z_index);
        GenericGameObject { base_properties, start, update, stop }
    }

    pub fn game_object_from_sprite(position: [f32; 3], texture_path: String, z_index: i32, start: fn(&mut System), update: fn(&mut System), stop: fn(&mut System)) -> Self {

        let base_properties = BaseGameObjectProperties::game_object_from_sprite(position, texture_path, z_index);
        GenericGameObject { base_properties, start, update, stop }
    }
    pub fn game_object_from_sprite_script(position: Array, texture_path: &str, z_index: i32) -> Self {

        let base_properties = BaseGameObjectProperties::game_object_from_sprite([position[0].clone_cast::<f64>() as f32,position[1].clone_cast::<f64>() as f32,position[2].clone_cast::<f64>() as f32], texture_path.to_string(), z_index);
        GenericGameObject { base_properties,start:| _system|{},update:| _system|{},stop:| _system|{} }
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
}
