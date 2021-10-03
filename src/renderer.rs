extern crate glium;

use glium::{Display, Surface};
pub use crate::window::Window;

pub struct Renderer{

}

impl Renderer {
pub fn new() -> Renderer {

    Renderer{
     
    }
}
pub fn render(&self, display: &Display){
    let mut target = display.draw();

        
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    
    target.finish().unwrap();
}


}