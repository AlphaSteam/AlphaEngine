#![allow(unused_imports)]
extern crate alpha_engine;
use alpha_engine::event::event_manager::EventManager;
use alpha_engine::event::{self, DeviceEvent, DeviceId, KeyboardInput, VirtualKeyCode};
use alpha_engine::{engine, game, shaders::Shader, sys, text};
use engine::Engine;
use game::Game;
use sys::{
    axes::Axis, cam::projection_ortho::ProjectionOrtho,
    cam::projection_perspective::ProjectionPerspective, fullscreen::Fullscreen,
    game_object::GameObject, system::System,
};
use text::font::Font;
fn start(system: &mut System, event_manager: &mut EventManager) {
    system.set_window_fullscreen(Fullscreen::False);
    system.set_window_resolution([600, 800]);
    system.set_window_maximized(true);
    system.set_current_shader(Shader::Basic);

    system.add_font("Arial", "src/fonts/ArialCE.ttf");
    system.render_text(
        "Test".to_string(),
        "Arial".to_string(),
        [200.0, 200.0],
        [10.0, 10.0],
        0.0,
        [1.0, 1.0, 1.0],
    );
    //let projection = ProjectionPerspective::new(0.6, 120.0, 0.0, 800.0);
    //system.camera_mut().set_projection(projection);
    //let window_resolution = system.get_window_resolution();
    /*   let projection = ProjectionOrtho::new(
        0.0,
        window_resolution[0],
        0.0,
        window_resolution[1],
        -500.0,
        500.0,
    ); */
    //system.camera_mut().set_projection(projection);
    /*  system
    .camera_mut()
    .transform_mut()
    .rotate(Axis::YAxis, -90.0); */

    let mut sprite = GameObject::game_object_from_sprite(
        [500.0, 200.0, 0.0],
        "src/sprites/placeholder.png".to_string(),
    );
    sprite.transform_mut().scale([1.0, 1.5, 1.0]);
    sprite.transform_mut().rotate(Axis::ZAxis, -90.0);
    //sprite.transform_mut().rotate(Axis::XAxis, 90.0);
    //sprite.transform_mut().rotate(Axis::YAxis, 90.0);

    system.add_game_object(sprite);

    let sprite2 = GameObject::game_object_from_sprite(
        [500.0, 200.0, 0.0],
        "src/sprites/placeholder.png".to_string(),
    );

    system.add_game_object(sprite2);
    event_manager.set_key_callback(process_inputs);
    event_manager.set_device_added_callback(device_added);
    event_manager.set_device_removed_callback(device_removed);
    event_manager.set_motion_callback(motion);
    event_manager.set_mouse_motion_callback(mouse_motion);
}
fn update(system: &mut System, _event_manager: &mut EventManager, _time_step: f32) {
    let object_transform = system
        .game_objects_mut()
        .first_mut()
        .unwrap()
        .transform_mut();

    object_transform.translate([1.0, 0.0, 0.0])
}
fn stop(_system: &mut System, _event_manager: &mut EventManager) {}

fn process_inputs(system: &mut System, key: KeyboardInput, _device_id: DeviceId) {
    let key_code = key.virtual_keycode;
    match key_code {
        None => println!("Key not recognized"),
        Some(virtual_key) => match virtual_key {
            VirtualKeyCode::D => match key.state {
                alpha_engine::event::ElementState::Pressed => println!("D pressed"),
                alpha_engine::event::ElementState::Released => println!("D released"),
            },
            VirtualKeyCode::P => match key.state {
                alpha_engine::event::ElementState::Pressed => system.set_framerate_target(1),
                alpha_engine::event::ElementState::Released => system.set_framerate_target(60),
            },
            _ => (),
        },
    };
}
fn device_added(_device_id: DeviceId) {
    //println!("Device {:?} added.", device_id);
}
fn device_removed(_device_id: DeviceId) {
    //println!("Device {:?} removed.", device_id);
}
fn motion(_axis: u32, _value: f64, _device_id: DeviceId) {
    //println!("AxisMotion, {:?}, {:?}, {:?}", axis, value, device_id)
}
fn mouse_motion(_delta: (f64, f64), _device_id: DeviceId) {
    //println!("MouseMotion, {:?}, {:?}", delta, device_id)
}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Basic_game".to_string());
    engine.start_main_loop();
}
