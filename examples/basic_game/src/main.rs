#![allow(unused_imports)]
extern crate alpha_engine;
use alpha_engine::event::{DeviceEvent, KeyboardInput};
use alpha_engine::{engine, game, shaders::Shader, sys, text};
use engine::Engine;
use game::Game;
use sys::{
    axes::Axis, cam::projection_ortho::ProjectionOrtho,
    cam::projection_perspective::ProjectionPerspective, fullscreen::Fullscreen,
    game_object::GameObject, system::System,
};
use text::font::Font;
fn start(system: &mut System) {
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
}
fn update(system: &mut System, _time_step: f32) {
    let _window_size = system.get_window_resolution();
    let event_manager = system.event_manager_mut();
    event_manager.set_key_callback(process_inputs);
}
fn stop(_system: &mut System) {}

fn process_inputs(key: KeyboardInput, event: DeviceEvent) {
    println!("event, {:?}", event);
    println!("key, {:?}", key);
}
fn device_added(event: DeviceEvent) {
    println!("{:?}", event);
}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Basic_game".to_string());
    engine.start_main_loop();
}
