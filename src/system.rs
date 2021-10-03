use glium::Display;

pub use crate::game::Game;
pub use crate::renderer::Renderer;
pub use crate::window::Window;

pub struct System {
    game: Game,
    renderer: Renderer,
    display: Display,
}

impl System {
    pub fn new(game: Game, display: Display) -> System {
        let system = System {
            game,
            renderer: Renderer::new(),
            display: display,
        };

        system
    }
    pub fn start(&self) {
        self.game.start(self);
    }

    pub fn update(&mut self, time_step: f32) {
        self.game.update(self, time_step);
        self.renderer.render(&self.display);
    }
    pub fn stop(&self) {
        self.game.stop(self);
        self.renderer.stop();
    }
}
