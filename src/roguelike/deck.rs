use rand::{thread_rng, Rng};
use indexmap::IndexMap;

use crate::game_objects::{implementations::card::Card, game_object::GmObj};

#[derive(Clone)]
pub struct Deck{

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub removed: Vec<Card>,
}
impl Deck{

    pub fn new(deck: Vec<Card>)->Self{
        Deck{
            deck,
            hand: Vec::new(),
            removed: Vec::new(),
        }
    }
    pub fn create_empty_deck()->Self{
        Deck{
            deck: Vec::new(),
            hand: Vec::new(),
            removed: Vec::new(),

        }
    }
    pub fn create_random_deck(card_pool: IndexMap<String, Card>, quantity: usize, should_render: bool)->Self{
        let card_templates = card_pool.len();
        let mut rng = thread_rng();
        let mut deck = Vec::with_capacity(quantity);
        for _ in 0..quantity{
            let random_index: usize = rng.gen_range(0..card_templates);
            let card = card_pool.get_index(random_index).unwrap().1;
            let mut card = card.clone();
            card.base_properties_mut().set_should_render(should_render); 
            deck.push(card);
            
        }

        Deck{
            deck,
            hand: Vec::new(),
            removed: Vec::new(),
        }
    }
    
}