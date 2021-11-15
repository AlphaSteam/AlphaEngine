use alpha_engine::sys::game_object::{BaseGameObjectProperties, GameObject, GenericGameObject};

#[derive(Clone)]
pub struct Card {
    base_properties: BaseGameObjectProperties,
    cost: i32,
}
impl Card {
    pub fn new(base_properties: BaseGameObjectProperties, cost: i32) -> Self {
        Card {
            base_properties,
            cost,
        }
    }
    pub fn card_from_sprite(position: [f32; 3], texture_path: String, cost: i32) -> Self {
        let base_properties =
            BaseGameObjectProperties::game_object_from_sprite(position, texture_path);
        Card {
            base_properties,
            cost,
        }
    }
}
impl GameObject for Card {
    fn get_base_properties(&self) -> &BaseGameObjectProperties {
        &self.base_properties
    }
    fn get_base_properties_mut(&mut self) -> &mut BaseGameObjectProperties {
        &mut self.base_properties
    }
}
