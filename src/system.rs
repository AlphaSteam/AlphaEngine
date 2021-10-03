

use glium::{Display};

pub use crate::window::Window;
pub use crate::game::Game;
pub use crate::renderer::Renderer;

pub struct System  {
game:  Game,
renderer: Renderer,
display:  Display,
}

impl System  {

    pub fn new(game: Game , display: Display ) -> System {
    let system = System {
            game,
            renderer: Renderer::new(),
            display: display
        };
   
    system

    }
    pub fn start(&self){
        self.game.start(self);
    }
    

    pub fn update(&mut self){
        self.game.update(self, 0.0);
        self.renderer.render(&self.display);
    }

}