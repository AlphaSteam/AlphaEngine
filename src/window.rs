extern crate glium;
use glium::{glutin, Surface};

struct Window {
    event_loop: glutin::event_loop::EventLoop,
    display:  glium::Display,

}

impl Window {

fn new(&self) -> Window {

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    Window{
        event_loop: event_loop,
        display: display
    }
}

}