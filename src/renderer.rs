extern crate glium;

pub use crate::window::Window;
use glium::{Display, Surface};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    pub fn render(&self, display: &Display) {
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.finish().unwrap();
    }
}
