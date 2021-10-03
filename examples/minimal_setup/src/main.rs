extern crate alpha_engine;
use crate::alpha_engine::{engine, game, system, private_system};
use crate::engine::Engine;
use crate::game::Game;
use crate::system::System;


fn start(system: &System){
    
}
fn update(system: &System, time_step: f32){

}
fn stop(system: &System){
}
fn main() {
    
    let game = Game::new(start,update,stop);
    let engine = Engine::new(game);
    engine.start_main_loop();
}
