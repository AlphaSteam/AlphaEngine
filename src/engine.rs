
pub use crate::private_system::PrivateSystem;
pub use crate::game::Game;
pub struct Engine {
system: PrivateSystem  

}
impl  Engine  {

    pub fn new(game: Game ) -> Engine   {
        let system = PrivateSystem::new(game);
        system.start();
        Engine {
            system
        }
    }
    pub fn start_main_loop(self){

        self.system.start_main_loop();
    }

}