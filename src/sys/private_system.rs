use glium::Display;

pub use crate::game::Game;
pub use crate::rendering::renderer::Renderer;
pub use crate::window::Window;
pub use crate::sys::system::System;

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
    pub fn new(game: Game, display: Display) -> PrivateSystem {
        let system = PrivateSystem {
            game,
            renderer: Renderer::new(),
            display: display,
            system: System::new(),
        };

        system
    }
    pub fn start(&mut self) {
        let mut system = &mut self.system;
        self.game.start(system);
        self.renderer.start(&self.display);
    }

    pub fn update(&mut self, time_step: f32) {
        let mut system = &mut self.system;
        self.game.update(system, time_step);
        self.renderer.render(&self.display);
    }
    pub fn stop(&mut self) {
        let mut system = &mut self.system;
        self.game.stop(system);
        self.renderer.stop();
    }
   
}
