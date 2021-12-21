extern crate alpha_engine;

use std::time::Duration;

use alpha_engine::audio::audio_engine::{effects::reverb::Reverb};
use alpha_engine::audio::audio_engine::effects::{BaseEffect};

use alpha_engine::audio::audio_engine::source::generic::GenericSourceBuilder;
use alpha_engine::audio::audio_engine::source::{Status};
use alpha_engine::event::event_manager::EventManager;
use alpha_engine::event::{DeviceId, KeyboardInput};
use alpha_engine::game_objects::game_object::{GmObj};
use alpha_engine::game_objects::implementations::{generic_game_object::GenericGameObject, character::Character, card::Card};
use alpha_engine::helpers::math::array3_to_vec3;
use alpha_engine::{engine, game, shaders::Shader, sys,};
use alpha_engine::roguelike::deck::Deck;
use engine::Engine;
use game::Game;
use sys::{
    fullscreen::Fullscreen,
    system::System,
};
use alpha_engine::roguelike::card_pool::CardPool;
use alpha_engine::event::VirtualKeyCode;
use std::collections::HashMap;

fn start(system: &mut System, event_manager: &mut EventManager) {
    system.set_window_fullscreen(Fullscreen::False);
    system.set_window_maximized(true);
    system.set_current_shader(Shader::Basic);
    set_card_pools(system);
    setup_game_objects(system);

    event_manager.set_key_callback(process_inputs);
    /* event_manager.set_device_added_callback(device_added);
    event_manager.set_device_removed_callback(device_removed);
    event_manager.set_motion_callback(motion);
    event_manager.set_mouse_motion_callback(mouse_motion); */

    setup_sound(system);


   system.run_script(r#"

    /* let generic_object = game_object_from_sprites(
      [1000.0, 400.0, 0.0],
      #{"Base":"src/sprites/Cards/Red.png"},
      "Base",
       0,
       true,
       true
        );
    add_game_object(game_objects,"script object",generic_object); */

  "#.to_string());



}
fn update(system: &mut System, _event_manager: &mut EventManager) {

    let sound_context = system
        .get_sound_context("Basic context".to_string())
        .unwrap();
    let handle = system.get_sound_source("Moving".to_string()).unwrap();
    let old_position = sound_context
        .state()
        .source_mut(*handle)
        .spatial_mut()
        .position();
    if old_position[0] < 3.0 {
        sound_context
            .state()
            .source_mut(*handle)
            .spatial_mut()
            .set_position(array3_to_vec3([
                old_position[0] + 0.1,
                old_position[1],
                old_position[2],
            ]));
    } else {
        sound_context
            .state()
            .source_mut(*handle)
            .spatial_mut()
            .set_position(array3_to_vec3([-1.0, 0.0, 0.0]));
    }
}
fn stop(_system: &mut System, _event_manager: &mut EventManager) {}

fn process_inputs(system: &mut System, key: KeyboardInput, _device_id: DeviceId) {
    let key_code = key.virtual_keycode;
    match key_code {
        None => println!("Key not recognized"),
        Some(virtual_key) => match virtual_key {
            VirtualKeyCode::N => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                    let mut sprites = HashMap::new();
                    sprites.insert("Idle".to_string(), "src/sprites/Characters/Skeleton/Idle".to_string());
                    sprites.insert("Attack".to_string(), "src/sprites/Characters/Skeleton/Attack".to_string());    
                    let mut skeleton = Character::character_from_sprites(
                        [1000.0, 70.0, 0.0],
                        sprites,
                        "Idle".to_string(),
                        100,
                        100,
                        3,
                        3,
                        "".to_string(),
                        false,
                        0,
                        true,
                        |_system| {
                            
                        },
                        |_system| {
                            
                        },
                        |_system| {
                            
                        },
                        |_system| {
                            
                        },
                        
                    );
                
                    skeleton.base_properties_mut().transform_mut().set_local_scale([4.0, 4.0, 1.0]);
                
                    system.game_objects_mut().add_game_object("Skeleton2".to_string(), Box::new(skeleton));
                }
                _ => (),
            },
            VirtualKeyCode::M => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                   
                
                    system.game_objects_mut().game_objects_mut().remove_entry("Skeleton2");
                }
                _ => (),
            },
            VirtualKeyCode::Z => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                   
                
                    system.set_framerate_target(30.0);
                }
                _ => (),
            },
            VirtualKeyCode::X => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                   
                
                    system.set_framerate_target(60.0);
                }
                _ => (),
            },
            VirtualKeyCode::C => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                   
                
                    system.set_framerate_target(144.0);
                }
                _ => (),
            },
          
            _ => (),
        },
    };
}

fn setup_sound(system: &mut System) {
    let sound_context = system.create_sound_context();

    system.add_sound_context("Basic context".to_string(), sound_context);


    let sewer_source = system.add_sound_buffer_from_file(
        "sewer".to_string(),
        "src/audio/The Sewer Monster.wav".to_string(),
        false,
    );

    // Left spatial source
    let generic_left_source = GenericSourceBuilder::new()
        .with_buffer(sewer_source.clone())
        .with_status(Status::Playing)
        .with_gain(0.8)
        .with_looping(true)
        .with_pitch(1.0);
    let mut left_source = system.create_sound_source_from_generic(generic_left_source, true);
    let left_source_spatial = left_source.spatial_mut();
    left_source_spatial.set_position(array3_to_vec3([-100.0, 0.0, 0.0]));

    let handle = system.add_source_to_context("Basic context".to_string(), left_source);

    system.add_sound_source("Left".to_string(), handle);

    // Right spatial source
    let generic_right_source = GenericSourceBuilder::new()
        .with_buffer(sewer_source.clone())
        .with_status(Status::Playing)
        .with_gain(1.0)
        .with_looping(true)
        .with_pitch(1.25);

    let mut right_source = system.create_sound_source_from_generic(generic_right_source, true);
    let right_source_spatial = right_source.spatial_mut();
    right_source_spatial.set_position(array3_to_vec3([100.0, 0.0, 0.0]));

    let handle = system.add_source_to_context("Basic context".to_string(), right_source);

    system.add_sound_source("Right".to_string(), handle);

    // Moving spatial source
    let generic_moving = GenericSourceBuilder::new()
        .with_buffer(sewer_source)
        .with_status(Status::Paused)
        .with_gain(0.1)
        .with_looping(true)
        .with_pitch(2.0);

    let moving_source = system.create_sound_source_from_generic(generic_moving, true);
    let handle = system.add_source_to_context("Basic context".to_string(), moving_source);
    system.add_sound_source("Moving".to_string(), handle);

    let base_effect = BaseEffect::default();
    let mut reverb = Reverb::new(base_effect);
    reverb.set_decay_time(Duration::from_secs_f32(4.0));
    reverb.set_dry(1.5);
    reverb.set_gain(0.01);
    reverb.set_fc(90.0);
   
}
fn set_card_pools(system: &mut System){

    set_red_cards_pool(system);
    set_slimed_cards_pool(system);

}
fn set_red_cards_pool(system: &mut System){
    let mut red_cards_pool = CardPool::new();

    // Magic missile
    let mut sprites = HashMap::new();
    sprites.insert("Base".to_string(), "src/sprites/Cards/Red.png".to_string());

    let mut magic_missile = Card::card_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Base".to_string(),
        "Magic missile".to_string(),
        "Attack for 1 damage".to_string(),
        1,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    magic_missile.base_properties_mut().transform_mut().set_size([200.0, 300.0, 1.0]);
    red_cards_pool.cards_mut().insert("Magic missile".to_string(), magic_missile);


    // Energy bolt
    let mut sprites = HashMap::new();
    sprites.insert("Base".to_string(), "src/sprites/Cards/Red.png".to_string());

    let mut energy_bolt = Card::card_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Base".to_string(),
        "Energy bolt".to_string(),
        "Attack for 3 damage".to_string(),
        2,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    energy_bolt.base_properties_mut().transform_mut().set_size([200.0, 300.0, 1.0]);
    red_cards_pool.cards_mut().insert("Energy bolt".to_string(), energy_bolt);



    system.card_pools_mut().insert("Red".to_string(),red_cards_pool);
}

fn set_slimed_cards_pool(system: &mut System){
    let mut slimed_cards_pool = CardPool::new();

    // Magic missile
    let mut sprites = HashMap::new();
    sprites.insert("Base".to_string(), "src/sprites/Cards/Slimed.png".to_string());

    let mut slime_missile = Card::card_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Base".to_string(),
        "Slime missile".to_string(),
        "Attack for 3 damage. Receive 1 damage".to_string(),
        1,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    slime_missile.base_properties_mut().transform_mut().set_size([200.0, 300.0, 1.0]);
    slimed_cards_pool.cards_mut().insert("Magic missile".to_string(), slime_missile);


    // Energy bolt
    let mut sprites = HashMap::new();
    sprites.insert("Base".to_string(), "src/sprites/Cards/Slimed.png".to_string());

    let mut slime_bolt = Card::card_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Base".to_string(),
        "Slime bolt".to_string(),
        "Attack for 6 damage. Lose your turn".to_string(),
        2,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    slime_bolt.base_properties_mut().transform_mut().set_size([200.0, 300.0, 1.0]);
    slimed_cards_pool.cards_mut().insert("Slime bolt".to_string(), slime_bolt);



    system.card_pools_mut().insert("Slimed".to_string(),slimed_cards_pool);
}
fn setup_game_objects(system: &mut System) {

    set_background(system);
    set_characters(system);


    

}
fn set_characters(system: &mut System){
    let mut sprites = HashMap::new();
    sprites.insert("Idle".to_string(), "src/sprites/Characters/Skeleton/Idle".to_string());
    sprites.insert("Attack".to_string(), "src/sprites/Characters/Skeleton/Attack".to_string());    
    let mut skeleton = Character::character_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Idle".to_string(),
        100,
        100,
        3,
        3,
        "".to_string(),
        false,
        0,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    skeleton.base_properties_mut().transform_mut().set_local_scale([4.0, 4.0, 1.0]);
    system.render_text("Skeleton HP".to_string(), format!("HP: {}",skeleton.current_hp()).to_string(),[1500.0, 700.0], [100,200,100]);

    system.game_objects_mut().add_game_object("Skeleton".to_string(), Box::new(skeleton));

    let cards = system.card_pools().clone()["Red"].cards().clone();

    let mut sprites = HashMap::new();
    sprites.insert("Idle".to_string(), "src/sprites/Characters/Wizard/Idle".to_string());
    sprites.insert("Attack1".to_string(), "src/sprites/Characters/Wizard/Attack1".to_string());
    let deck = Deck::create_random_deck(cards, 10, true);
    system.decks_mut().insert("Wizard deck".to_string(),deck);
    let mut wizard = Character::character_from_sprites(
        [200.0, 120.0, 0.0],
        sprites,
        "Idle".to_string(),
        100,
        100,
        4,
        4,
        "Wizard deck".to_string(),
        true,
        0,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );
    wizard.base_properties_mut().transform_mut().set_local_scale([3.0, 3.0, 1.0]);
    system.render_text("Wizard HP".to_string(), format!("HP: {}",wizard.current_hp()).to_string(),[450.0, 700.0], [100,200,100]);
    system.render_text("Energy".to_string(), format!("Energy: {}", wizard.current_energy()).to_string(),[100.0, 1000.0], [100,200,100]);

    system.game_objects_mut().add_game_object("Wizard".to_string(), Box::new(wizard));

}
fn set_background(system: &mut System){
    let mut sprites = HashMap::new();
    sprites.insert("Base".to_string(), "src/sprites/bg.png".to_string());
    let mut bg = GenericGameObject::game_object_from_sprites(
        [0.0, 0.0, 0.0],
        sprites,
        "Base".to_string(),
        -9999,
        true,
        false,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );
    let window_resolution = system.get_window_resolution();

    bg
        .base_properties_mut()
        .transform_mut()
        .set_size([window_resolution[0], window_resolution[1], 1.0]);

    system.game_objects_mut().add_game_object("Background".to_string(), Box::new(bg));

   


}
fn main() {
    let game = Game::new(start, update, stop);
    let engine = Engine::new(game, "Basic game".to_string());
    engine.start_main_loop();
}
