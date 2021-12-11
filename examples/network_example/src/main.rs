extern crate alpha_engine;
use std::env;
use std::time::Duration;

use alpha_engine::event::event_manager::EventManager;
use alpha_engine::event::{DeviceId, KeyboardInput, VirtualKeyCode};

use crate::alpha_engine::{engine, game, sys};
use crate::engine::Engine;
use crate::game::Game;
use crate::sys::system::System;
use alpha_engine::net::Config;
fn start(system: &mut System, event_manager: &mut EventManager) {
    // We get the ips of client and server from the arguments passed in the command line. If not present, we use some default ones.
    event_manager.set_key_callback(process_inputs);

    let mut client_address = "127.0.0.1:12346";
    let mut server_address = "127.0.0.1:12345";
    let args: Vec<String> = env::args().collect();
    for (i, argument) in env::args().enumerate() {
        if argument == "--client-address" && args.len() > i + 1 {
            client_address = &args[i + 1];
        } else if argument == "--server-address" && args.len() > i + 1 {
            server_address = &args[i + 1];
        }
    }
    // We create the config and set the number of seconds before a timeout is set.
    let mut config = Config::default();
    config.idle_connection_timeout = Duration::new(1, 0);
    // We create the client by binding a socket to the provided ip.
    system.connect_to_network_with_config(
        server_address.parse().unwrap(),
        client_address.parse().unwrap(),
        config,
    )
}
fn update(_system: &mut System, _event_manager: &mut EventManager) {}
fn stop(_system: &mut System, _event_manager: &mut EventManager) {}

fn process_inputs(system: &mut System, key: KeyboardInput, _device_id: DeviceId) {
    let key_code = key.virtual_keycode;
    match key_code {
        None => println!("Key not recognized"),
        Some(virtual_key) => match virtual_key {
            VirtualKeyCode::S => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    // If S is pressed, we send a package with a vector to the server ip provided when starting the program.
                    let _res = system.send_packet(vec![1, 2, 3]);
                }
                _ => (),
            },
            VirtualKeyCode::D => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    // If D is pressed, we send a number each second to the server ip provided when starting the program.
                    let _res = system.test_packets();
                }
                _ => (),
            },
            _ => (),
        },
    }
}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Minimal setup".to_string());
    engine.start_main_loop();
}
