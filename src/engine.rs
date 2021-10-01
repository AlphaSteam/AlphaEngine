
mod system;
pub use crate::system::System;
struct Engine {
system: System

}

impl Engine {

    fn new(&self, system: System) -> Engine{
        Engine {
            system
        }
    }

}