extern crate glium;

pub use crate::sys::private_system::PrivateSystem;
use glium::glutin;
use glutin::{Api, GlProfile, GlRequest};
use glium::backend::glutin::Display;
pub struct Window {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub display: Display,
}

impl Window {
    pub fn new(win_title: String) -> Window {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_maximized(true)
            .with_title(win_title);

        let context_builder = glutin::ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_srgb(true)
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)));
        //.with_vsync(true);
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
        
        Window {
            event_loop,
            display,
        }
    }

    pub fn update(&self) {}
}
