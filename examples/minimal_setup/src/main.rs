extern crate alpha_engine;
use crate::alpha_engine::{engine, game, sys};
use crate::engine::Engine;
use crate::game::Game;
use crate::sys::system::System;

fn start(system: &mut System) {}
fn update(system: &mut System, time_step: f32) {}
fn stop(system: &mut System) {}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game);
    engine.start_main_loop();
}
