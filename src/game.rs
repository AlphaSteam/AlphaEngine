pub use crate::sys::private_system::PrivateSystem;
use crate::sys::system::System;

pub struct Game {
    user_start: fn(system: &mut System),
    user_update: fn( system: &mut System, time_step: f32),
    user_stop: fn(system: &mut System),
}

impl Game {
    pub fn new(
        start: fn(system: &mut System),
        update: fn( system: &mut System, time_step: f32),
        stop: fn(system: &mut System ),
    ) -> Game {
        Game {
            user_start: start,
            user_update: update,
            user_stop: stop,
        }
    }
    pub fn start(&self, system: &mut System) {
        (self.user_start)(system);
    }
    pub fn update(&self, system: &mut System, time_step: f32) {
        (self.user_update)(system, time_step);
    }
    pub fn stop(&self, system: &mut System) {
        (self.user_stop)(system);
    }
}
