use std::time::{Duration, Instant};

use crate::event::event_manager::EventManager;
pub use crate::game::Game;
pub use crate::sys::private_system::PrivateSystem;
use crate::window::Window;
use egui_glium::EguiGlium;
use glutin::event_loop::EventLoop;
pub struct Engine {
    event_loop: EventLoop<()>,
    private_system: PrivateSystem,
    event_manager: EventManager,
    egui: EguiGlium,
}
impl Engine {
    pub fn new(game: Game, win_title: String) -> Self {
        let window = Window::new(win_title);
        let display = window.display;
        let event_loop = window.event_loop;
        let mut event_manager = EventManager::new();
        let mut private_system = PrivateSystem::new(game, display.clone());
        let mut egui = egui_glium::EguiGlium::new(&display);
        private_system.start(&mut egui,&mut event_manager);
        let engine = Engine {
            event_loop,
            private_system,
            event_manager,
            egui,
        };

        engine
    }

    pub fn start_main_loop(self) {
        let mut private_system = self.private_system;
        let mut event_manager = self.event_manager.clone();
        let display = private_system.display().clone();
        
    
        let mut old_render = Instant::now();
        let mut old_frame = Instant::now();
        let mut remaining_time = private_system.system().frame_time_target_nanos();
        let mut egui = self.egui;
        self.event_loop.run(move |ev, _, control_flow| {
            let now = Instant::now();

            //private_system.system().frame_time_target_nanos()
            let next_frame_time = now + Duration::from_nanos(remaining_time);

            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => {
                    egui.on_event(&event);

                    match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            private_system.stop( &mut event_manager);
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
                        } => {
                            event_manager.run_axis_motion_callback(axis, value, device_id);
                        }
                        glutin::event::WindowEvent::Focused(val) => {
                            *event_manager.window_focused_mut() = val
                        }
                        _ => {
                            display.gl_window().window().request_redraw();
                        }
                    }
                }
                glutin::event::Event::DeviceEvent {
                    event, device_id, ..
                } => {
                    if *event_manager.window_focused() == true {
                        match event {
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
                        }
                    }
                }

                glutin::event::Event::MainEventsCleared => {
                    let now = Instant::now();

                    let time = now.duration_since(old_frame);

                    if time.as_nanos() as u64 >= remaining_time {
                        private_system.update(&mut egui, &mut event_manager, &mut old_render);
                        old_render = now;
                        let extra_time = time.as_nanos() as u64 - remaining_time;
                        if extra_time < private_system.system().frame_time_target_nanos() {
                            remaining_time =
                                private_system.system().frame_time_target_nanos() - extra_time;
                        }
                    } else {
                        remaining_time -= time.as_nanos() as u64;
                    }
                    old_frame = now;
                }

                _ => (),
            }
        });
    }
}
