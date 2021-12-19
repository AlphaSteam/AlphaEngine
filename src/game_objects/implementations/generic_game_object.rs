use rhai::Array;

use crate::{game_objects::game_object::{BaseGameObjectProperties, GmObj}, sys::system::System, rendering::mesh::Mesh};
use crate::image::GenericImageView;

#[derive(Clone)]
pub struct GenericGameObject {
    base_properties: BaseGameObjectProperties,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
}
impl GenericGameObject {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture_path: String, start: fn(&mut System), update: fn(&mut System), stop: fn(&mut System)) -> Self {
        let base_properties = BaseGameObjectProperties::new(position, size, mesh, texture_path);
        GenericGameObject { base_properties, start, update, stop }
    }

    pub fn game_object_from_sprite(position: [f32; 3], texture_path: String, start: fn(&mut System), update: fn(&mut System), stop: fn(&mut System)) -> Self {
        let mesh = Mesh::create_rectangle();

        let texture = image::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => texture,
            Err(_) => {
                image::load_from_memory(include_bytes!("../../assets/sprites/default.png")).unwrap()
            }
        };
        let texture_w = texture.width() as f32;
        let texture_h = texture.height() as f32;
        GenericGameObject::new(position, [texture_w, texture_h, 1.0], mesh, texture_path, start, update, stop)
    }
    pub fn game_object_from_sprite_script(position: Array, texture_path: &str) -> Self {
        let mesh = Mesh::create_rectangle();

        let texture = image::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => texture,
            Err(_) => {
                image::load_from_memory(include_bytes!("../../assets/sprites/default.png")).unwrap()
            }
        };
        let texture_w = texture.width() as f32;
        let texture_h = texture.height() as f32;

     
        GenericGameObject::new([position[0].clone_cast::<f64>() as f32,position[1].clone_cast::<f64>() as f32,position[2].clone_cast::<f64>() as f32],
         [texture_w, texture_h, 1.0], mesh, texture_path.to_string(),| _system|{},| _system|{},| _system|{})
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
