

mod window;
pub use crate::window::Window;

mod game;
pub use crate::game::Game;



struct System {
window: Window,
game: Game,
should_close: Boolean

}

impl Engine {

    fn new(&self, game: Game) -> Engine{
    let engine = Engine {
            window: Window::new(),
            game,
            should_close: false
        };
    window.start();
    game.start();
    engine
    }

}