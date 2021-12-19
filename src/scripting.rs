

use rhai::{plugin::*, Scope};

use crate::{sys::game_objects::GameObjects, game_objects::{implementations::generic_game_object::GenericGameObject, game_object::GameObject}};
pub struct ScriptEngine<'a> {
    pub engine: Engine,
    //pub scripts: ScriptFunctions,
    pub scope: Scope<'a>,
}
impl ScriptEngine<'_> {
    pub fn new(  game_objects: GameObjects) -> Self {
        let mut engine = Engine::new();
        // The macro call creates a Rhai module from the plugin module.
        //let module = exported_module!(script_functions);

        // A module can simply be registered into the global namespace.
        //engine.register_static_module("rust_callbacks", module.into());
        engine
            .register_type_with_name::<Box<dyn GameObject>>("GameObject")
            .register_fn("game_object_from_sprite", GenericGameObject::game_object_from_sprite_script);

        
        let mut scope = Scope::new();
        scope.push("t", game_objects);

 
        //let scripts = ScriptFunctions::new(game_objects);
        ScriptEngine { engine, scope }
    }

}



