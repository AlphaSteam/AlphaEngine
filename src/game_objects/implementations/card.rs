use std::collections::HashMap;
use crate::{game_objects::game_object::{BaseGameObjectProperties, GmObj}, sys::system::System};

#[derive(Clone)]
pub struct Card {
    base_properties: BaseGameObjectProperties,
    name: String,
    description: String,
    cost: i32,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
    action:  fn(&mut System),

}
impl Card {
    pub fn new(
        base_properties: BaseGameObjectProperties,
        name: String,
        description: String,
        cost: i32,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action:  fn(&mut System),

    ) -> Self {
      
        Card {
            base_properties,
            name,
            description,
            cost,
            start,
            update,
            stop,
            action
        }
    }
    pub fn card_from_sprites(
        position: [f32; 3],
        texture_paths: HashMap<String, String>,
        default_texture:String,
        name: String,
        description: String,
        cost: i32,
        should_render: bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action:  fn(&mut System),
    ) -> Self {
        let base_properties =
            BaseGameObjectProperties::game_object_from_sprites(
                position,
                texture_paths,
                default_texture,
                    0,
                should_render,
                true,
            );
        Card {
            base_properties,
            name,
            description,
            cost,
            start,
            update,
            stop,
            action
        }
    }
    pub fn cost(&self) -> i32 {
        self.cost
    }
    pub fn name(self) -> String {
        self.name
    }
    pub fn description(self) -> String {
        self.description
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
    fn action(&mut self) -> fn(&mut System) {
        self.action
    }
}


