pub use crate::system::System;

pub struct Game {
user_start: fn(system: &System),
user_update: fn(system: &System, time_step: f32),
user_stop: fn(system: &System),
}

impl  Game  {

    pub fn new( start: fn(system: &System) , update: fn(system: &System, time_step: f32), stop: fn(system: &System)) -> Game {

        Game{
            user_start: start,
            user_update: update,
            user_stop: stop
        }

    }
    pub fn start(&self,system: &System){
        (self.user_start)(system);
        
    }

}