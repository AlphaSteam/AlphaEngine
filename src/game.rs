pub use crate::sys::private_system::PrivateSystem;
use crate::{event::event_manager::EventManager, sys::system::System, scripting::ScriptEngine};

pub struct Game {
    user_start: fn(system: &mut System, &mut EventManager, script_engine: &mut ScriptEngine),
    user_update: fn(system: &mut System, &mut EventManager, script_engine: &mut ScriptEngine),
    user_stop: fn(system: &mut System, &mut EventManager, script_engine: &mut ScriptEngine),
}

impl Game {
    pub fn new(
        start: fn(system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine),
        update: fn(system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine),
        stop: fn(system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine),
    ) -> Game {
        Game {
            user_start: start,
            user_update: update,
            user_stop: stop,
        }
    }
    pub fn start(&self, system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine) {
        (self.user_start)(system, event_manager, script_engine);
    }
    pub fn update(&self, system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine) {
        (self.user_update)(system, event_manager, script_engine);
    }
    pub fn stop(&self, system: &mut System, event_manager: &mut EventManager, script_engine: &mut ScriptEngine) {
        (self.user_stop)(system, event_manager, script_engine);
    }
}
