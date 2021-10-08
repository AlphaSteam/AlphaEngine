extern crate glium;

pub use crate::sys::private_system::PrivateSystem;
use glium::glutin;

pub struct Window {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
}

impl Window {
    pub fn new() -> Window {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new();
        let context_builder = glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        Window {
            event_loop,
            display,
        }
    }

    pub fn update(&self) {}
}
