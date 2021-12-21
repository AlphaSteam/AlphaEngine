use indexmap::IndexMap;

use crate::game_objects::implementations::card::Card;

pub struct CardPool{
    cards: IndexMap<String, Card>,
}
impl CardPool{
    pub fn new()-> Self{
        CardPool{
            cards: IndexMap::new(),
        }
    }

    pub fn cards(&self)->& IndexMap<String,Card>{
        &self.cards
    }

    pub fn cards_mut(&mut self)->&mut IndexMap<String,Card>{
        &mut self.cards
    }
}