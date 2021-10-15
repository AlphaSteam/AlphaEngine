use std::time::{Duration, Instant};

use glutin::event::DeviceId;
use glutin::event_loop::EventLoop;

use crate::event::event_manager::EventManager;
pub use crate::game::Game;
pub use crate::sys::private_system::PrivateSystem;
use crate::window::Window;

pub struct Engine {
    event_loop: EventLoop<()>,
    private_system: PrivateSystem,
}
impl Engine {
    pub fn new(game: Game, win_title: String) -> Engine {
        let window = Window::new(win_title);
        let display = window.display;
        let event_loop = window.event_loop;
        let event_manager = EventManager::new();
        let mut private_system = PrivateSystem::new(game, display, event_manager);
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
                glutin::event::Event::DeviceEvent { event, .. } => match event {
                    /*

                    glutin::event::DeviceEvent::MouseMotion { delta } => todo!(),
                    glutin::event::DeviceEvent::MouseWheel { delta } => todo!(),
                    glutin::event::DeviceEvent::Motion { axis, value } => {}
                    glutin::event::DeviceEvent::Button { button, state } => todo!(),

                    glutin::event::DeviceEvent::Text { codepoint } => todo!(), */
                    glutin::event::DeviceEvent::Removed => {
                        println!("{:?}", unsafe { DeviceId::dummy() })
                    }
                    glutin::event::DeviceEvent::Added => private_system
                        .system()
                        .event_manager()
                        .run_device_added_callback(event),
                    glutin::event::DeviceEvent::Key(key) => private_system
                        .system()
                        .event_manager()
                        .run_key_callback(key, event),
                    _ => (),
                },
                _ => (),
            }

            private_system.update(next_frame_time_f32);
        });
    }
}
