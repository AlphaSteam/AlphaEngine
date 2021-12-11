
use crate::sys::{game_object::{GameObject, GenericGameObject}, game_objects::{GameObjects}};
use rhai::plugin::*;
pub struct ScriptEngine {
    pub engine: Engine,
    pub scripts: ScriptFunctions,
}
impl ScriptEngine {
    pub fn new(  game_objects: GameObjects) -> Self {
        let mut engine = Engine::new();

        // The macro call creates a Rhai module from the plugin module.
        //let module = exported_module!(script_functions);

        // A module can simply be registered into the global namespace.
        //engine.register_static_module("rust_callbacks", module.into());
        engine
            .register_type_with_name::<Box<dyn GameObject>>("GameObject")
            .register_fn("game_object_from_sprite", GenericGameObject::game_object_from_sprite_script);
        let scripts = ScriptFunctions::new(game_objects);
        ScriptEngine { engine, scripts }
    }
}

pub struct ScriptFunctions{
    game_objects: GameObjects
}
impl ScriptFunctions{
    pub fn new(game_objects: GameObjects)->Self{
        ScriptFunctions{game_objects}
    }
    pub fn add_game_object(&mut self, game_object_id: String, game_object: Box<dyn GameObject>) {
        self.game_objects.add_game_object(game_object_id,game_object)
    }
}

