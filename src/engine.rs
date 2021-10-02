
pub use crate::system::System;
pub use crate::game::Game;
pub struct Engine{
system: System

}
impl Engine{

    pub fn new(game: Game) -> Engine{
      
        Engine {
            system:  System::new(game)
        }
    }
    pub fn start_main_loop(&self){

        self.system.start_main_loop();
    }

}