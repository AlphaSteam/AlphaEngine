use std::time::{Duration, Instant};

use glutin::event_loop::EventLoop;

use crate::event::event_manager::EventManager;
pub use crate::game::Game;
pub use crate::sys::private_system::PrivateSystem;
use crate::window::Window;

pub struct Engine {
    event_loop: EventLoop<()>,
    private_system: PrivateSystem,
    event_manager: EventManager,
}
impl Engine {
    pub fn new(game: Game, win_title: String) -> Engine {
        let window = Window::new(win_title);
        let display = window.display;
        let event_loop = window.event_loop;
        let mut event_manager = EventManager::new();
        let mut private_system = PrivateSystem::new(game, display);
        private_system.start(&mut event_manager);
        let engine = Engine {
            event_loop,
            private_system,
            event_manager,
        };

        engine
    }

    pub fn start_main_loop(self) {
        let mut private_system = self.private_system;
        let mut event_manager = self.event_manager.clone();
        self.event_loop.run(move |ev, _, control_flow| {
            let next_frame_time = Instant::now()
                + Duration::from_nanos(private_system.system().frame_time_target_nanos());

            let next_frame_time_f32 = Instant::now().elapsed().as_secs_f32()
                + Duration::from_nanos(private_system.system().frame_time_target_nanos())
                    .as_secs_f32();
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::MouseInput {
                        state,
                        button,
                        device_id,
                        ..
                    } => event_manager.run_mouse_input_callback(state, button, device_id),
                    glutin::event::WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => event_manager.run_cursor_moved_callback(position, device_id),
                    glutin::event::WindowEvent::AxisMotion {
                        axis,
                        value,
                        device_id,
                    } => event_manager.run_axis_motion_callback(axis, value, device_id),
                    _ => return,
                },
                glutin::event::Event::DeviceEvent {
                    event, device_id, ..
                } => match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => {
                        event_manager.run_mouse_motion_callback(delta, device_id)
                    }
                    glutin::event::DeviceEvent::MouseWheel { delta } => {
                        event_manager.run_mouse_wheel_callback(delta, device_id)
                    }
                    glutin::event::DeviceEvent::Motion { axis, value } => {
                        event_manager.run_motion_callback(axis, value, device_id)
                    }
                    glutin::event::DeviceEvent::Button { button, state } => {
                        event_manager.run_button_callback(button, state, device_id)
                    }
                    glutin::event::DeviceEvent::Text { codepoint } => {
                        event_manager.run_text_callback(codepoint, device_id)
                    }
                    glutin::event::DeviceEvent::Removed => {
                        event_manager.run_device_removed_callback(device_id)
                    }
                    glutin::event::DeviceEvent::Added => {
                        event_manager.run_device_added_callback(device_id)
                    }
                    glutin::event::DeviceEvent::Key(key) => event_manager.run_key_callback(
                        &mut private_system.system_mut(),
                        key,
                        device_id,
                    ),
                },
                _ => (),
            }

            private_system.update(&mut event_manager, next_frame_time_f32);
        });
    }
}
