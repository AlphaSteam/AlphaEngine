pub use crate::sys::private_system::PrivateSystem;
use crate::{event::event_manager::EventManager, sys::system::System};

pub struct Game {
    user_start: fn(system: &mut System, &mut EventManager),
    user_update: fn(system: &mut System, &mut EventManager),
    user_stop: fn(system: &mut System, &mut EventManager),
}

impl Game {
    pub fn new(
        start: fn(system: &mut System, event_manager: &mut EventManager),
        update: fn(system: &mut System, event_manager: &mut EventManager),
        stop: fn(system: &mut System, event_manager: &mut EventManager),
    ) -> Game {
        Game {
            user_start: start,
            user_update: update,
            user_stop: stop,
        }
    }
    pub fn start(&self, system: &mut System, event_manager: &mut EventManager) {
        (self.user_start)(system, event_manager);
    }
    pub fn update(&self, system: &mut System, event_manager: &mut EventManager) {
        (self.user_update)(system, event_manager);
    }
    pub fn stop(&self, system: &mut System, event_manager: &mut EventManager) {
        (self.user_stop)(system, event_manager);
    }
}
