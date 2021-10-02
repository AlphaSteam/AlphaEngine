
use glium::Surface;

pub use crate::window::Window;
pub use crate::game::Game;
use std::time::{Duration, Instant};



pub struct System{
window: Window,
game: Game,
should_close: bool,
}

impl System{

    pub fn new(game: Game) -> System{
    let system = System {
            window: Window::new(),
            game,
            should_close: false
        };
    system.game.start();
    system
    }
    pub fn start_main_loop(self){
        self.window.event_loop.run(move |ev, _, control_flow| {
            let next_frame_time = Instant::now() +
            Duration::from_nanos(16_666_667);
            let next_frame_time_f32 = Instant::now().elapsed().as_secs_f32() + Duration::from_nanos(16_666_667).as_secs_f32();
            println!("{}",next_frame_time_f32);
            let mut target = self.window.display.draw();
    
            
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
}