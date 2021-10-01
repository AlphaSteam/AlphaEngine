extern crate glium;
use glium::{glutin, Surface};
use std::time::{Duration, Instant};

mod window;

fn main() {
    

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = Instant::now() +
        Duration::from_nanos(16_666_667);
        let next_frame_time_f32 = Instant::now().elapsed().as_secs_f32() + Duration::from_nanos(16_666_667).as_secs_f32();
        println!("{}",next_frame_time_f32);
        let mut target = display.draw();

        
        target.clear_color(next_frame_time_f32, next_frame_time_f32, 1.0, 1.0);

        target.finish().unwrap();

       
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
