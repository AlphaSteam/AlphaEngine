mod system;

pub use crate::system::System;

struct Game {
pub start: fn(system: System),
pub update: fn(system: System),
pub stop: fn(system: System),

}

impl Game {

    fn new(&self, start: fn(system: System), update: fn(system: System), stop: fn(system: System)) -> Game {

        Game{
            start,
            update,
            stop
        }

    }

}