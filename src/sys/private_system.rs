use glium::Display;

pub use crate::game::Game;
pub use crate::rendering::renderer::Renderer;
pub use crate::sys::system::System;
pub use crate::window::Window;

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
        let system = System::new(display.clone());
        let private_system = PrivateSystem {
            game,
            renderer: Renderer::new(display.clone()),
            display,
            system,
        };

        private_system
    }
    pub fn start(&mut self) {
        let system = &mut self.system;
        self.game.start(system);
        self.renderer.start(&self.display, system);
    }

    pub fn update(&mut self, time_step: f32) {
        let system = &mut self.system;
        self.game.update(system, time_step);
        self.renderer.render(&self.display, system);
    }
    pub fn stop(&mut self) {
        let system = &mut self.system;
        self.game.stop(system);
        self.renderer.stop();
    }
}
