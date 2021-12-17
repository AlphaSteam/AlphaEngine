use std::{time::Instant};

use crate::event::event_manager::EventManager;
pub use crate::game::Game;
pub use crate::rendering::renderer::Renderer;
pub use crate::window::Window;
use egui_glium::EguiGlium;
use glium::Display;

use super::system::System;

/**
Struct that hosts the engine functions


*/

pub struct PrivateSystem {
    game: Game,
    renderer: Renderer,
    display: Display,
    system: System,
}

impl PrivateSystem {
    pub fn new(game: Game, display: Display) -> Self {
        let system = System::new(display.clone());
        let game_objects = system.game_objects().clone();
        //let script_engine = ScriptEngine::new(game_objects);
        /* let _thread = thread::spawn(move || {
            let err = script_engine.engine.run(r#"
            let generic_object = game_object_from_sprite(
                [0.0,0.0,0.0],
                "asdasd");
            rust_callbacks::add_game_object(generic_object);
            "#);
            
        }); */
        
        let private_system = PrivateSystem {
            game,
            renderer: Renderer::new(),
            display,
            system,
            
        };
 
        private_system
    }
    pub fn start(&mut self, event_manager: &mut EventManager) {
        let system = &mut self.system;
        self.game.start(system, event_manager);
        self.renderer.start(&self.display, system);
    }

    pub fn update(
        &mut self,
        egui: &mut EguiGlium,
        event_manager: &mut EventManager,
        old_render: &mut Instant,
    ) {
        let system = &mut self.system;
        self.game.update(system, event_manager);
        self.renderer
            .render(&self.display, egui, system, old_render);
    }
    pub fn stop(&mut self, event_manager: &mut EventManager) {
        let system = &mut self.system;
        self.game.stop(system, event_manager);
        self.renderer.stop();
    }
    pub fn system(&self) -> &System {
        &self.system
    }
    pub fn system_mut(&mut self) -> &mut System {
        &mut self.system
    }
    pub fn display(&self) -> &Display {
        &self.display
    }
    
}
