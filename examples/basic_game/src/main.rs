extern crate alpha_engine;
extern crate nalgebra_glm as glm;
use alpha_engine::{engine, game, sys};
use engine::Engine;
use game::Game;
use sys::{game_object::GameObject, system::System, transform::Transform};

fn start(system: &mut System) {
    let transform = Transform::new(
        glm::vec3(1.0, 0.0, 0.0),
        glm::quat(0.0, 2.0, 0.0, 0.0),
        glm::vec3(2.0, 0.0, 0.0),
    );
    let sprite = GameObject::game_object_from_sprite(transform);
    system.add_game_object(sprite);
}
fn update(system: &mut System, _time_step: f32) {}
fn stop(system: &mut System) {}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game);
    engine.start_main_loop();
}
