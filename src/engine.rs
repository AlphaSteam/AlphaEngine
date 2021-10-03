pub use crate::game::Game;
pub use crate::private_system::PrivateSystem;
pub struct Engine {
    private_system: PrivateSystem,
}
impl Engine {
    pub fn new(game: Game) -> Engine {
        let private_system = PrivateSystem::new(game);
        private_system.start();
        Engine { private_system }
    }
    pub fn start_main_loop(self) {
        self.private_system.start_main_loop();
    }
}
