use std::time::{Duration, Instant};

use glutin::event_loop::EventLoop;

pub use crate::game::Game;
pub use crate::sys::private_system::PrivateSystem;
use crate::window::Window;

pub struct Engine {
    event_loop: EventLoop<()>,
    private_system: PrivateSystem,
}
impl Engine {
    pub fn new(game: Game) -> Engine {
        let window = Window::new();
        let display = window.display;
        let event_loop = window.event_loop;
        let mut private_system = PrivateSystem::new(game, display);
        private_system.start();
        let engine = Engine {
            event_loop,
            private_system,
        };

        engine
    }

    pub fn start_main_loop(self) {
        let mut private_system = self.private_system;
        self.event_loop.run(move |ev, _, control_flow| {
            let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);

            let next_frame_time_f32 = Instant::now().elapsed().as_secs_f32()
                + Duration::from_nanos(16_666_667).as_secs_f32();
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }

            private_system.update(next_frame_time_f32);
        });
    }
}
