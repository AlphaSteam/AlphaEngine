pub use crate::system::System;

pub struct Game{
pub start: fn(system: System),
pub update: fn(system: System, time_step: f32),
pub stop: fn(system: System),
}

impl Game {

    pub fn new( start: fn(system: System), update: fn(system: System, time_step: f32), stop: fn(system: System)) -> Game {

        Game{
            start,
            update,
            stop
        }

    }
    pub fn start(&self){
        println!("Starting game")
    }

}