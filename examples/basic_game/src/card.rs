use alpha_engine::{sys::system::System, game_objects::game_object::{BaseGameObjectProperties, GmObj}};


#[derive(Clone)]
pub struct Card {
    base_properties: BaseGameObjectProperties,
    cost: i32,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
}
impl Card {
    pub fn new(base_properties: BaseGameObjectProperties, cost: i32, start: fn(&mut System),update: fn(&mut System), stop: fn(&mut System)) -> Self {
      
        Card {
            base_properties,
            cost,
            start,
            update,
            stop,
        }
    }
    pub fn card_from_sprite(position: [f32; 3], texture_path: String, cost: i32, start: fn(&mut System),update: fn(&mut System), stop: fn(&mut System)) -> Self {
        let base_properties =
            BaseGameObjectProperties::game_object_from_sprite(position, texture_path);
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
