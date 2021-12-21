use alpha_engine::{sys::system::System, game_objects::game_object::{BaseGameObjectProperties, GmObj}};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Card {
    base_properties: BaseGameObjectProperties,
    cost: i32,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
}
impl Card {
    pub fn new(
        base_properties: BaseGameObjectProperties,
        cost: i32,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System)
    ) -> Self {
      
        Card {
            base_properties,
            cost,
            start,
            update,
            stop,
        }
    }
    pub fn card_from_sprites(position: [f32; 3],
        texture_paths: HashMap<String, String>,
        default_texture:String,
        cost: i32,z_index: i32,
        should_render: bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System)
    ) -> Self {
        let base_properties =
            BaseGameObjectProperties::game_object_from_sprites(
                position,
                texture_paths,
                default_texture,
                z_index,
                should_render,
            );
        Card {
            base_properties,
            cost,
            start,
            update,
            stop
        }
    }
    pub fn cost(&self) -> i32 {
        self.cost
    }
}
impl GmObj for Card {
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
