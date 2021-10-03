
use glium::Surface;

pub use crate::window::Window;
pub use crate::game::Game;



pub struct System  {
window: Window,
game:  Game ,
should_close: bool,
}

impl System {

    pub fn new(game: Game ) -> System{
    let system = System {
            window: Window::new(),
            game,
            should_close: false
        };
   
    system

    }
    pub fn start(&self){
        self.game.start(self);
    }
    fn update(&self, time_step: f32){
        println!("{}",time_step);
        let mut target = self.window.display.draw();

        
        target.clear_color(time_step, time_step, 1.0, 1.0);

        target.finish().unwrap();
    }
    pub fn start_main_loop(self){
    
    let window = self.window;
    let event_loop =  window.event_loop;
    let display = window.display;

    event_loop.run(move  |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

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