extern crate alpha_engine;
use alpha_engine::event::event_manager::EventManager;

use crate::alpha_engine::{engine, game, sys};
use crate::engine::Engine;
use crate::game::Game;
use crate::sys::system::System;

fn start(_system: &mut System, _event_manager: &mut EventManager) {}
fn update(_system: &mut System, _event_manager: &mut EventManager) {}
fn stop(_system: &mut System, _event_manager: &mut EventManager) {}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Minimal setup".to_string());
    engine.start_main_loop();
}
