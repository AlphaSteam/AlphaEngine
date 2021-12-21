#![allow(unused_imports)]
pub mod card;

extern crate alpha_engine;
use std::any::Any;
use std::borrow::Borrow;
use std::thread;
use std::time::Duration;

use alpha_engine::audio::audio_engine::algebra::{UnitQuaternion, Vector3};
use alpha_engine::audio::audio_engine::buffer::{DataSource, SoundBufferResource};
use alpha_engine::audio::audio_engine::context::SoundContext;
use alpha_engine::audio::audio_engine::effects::reverb::Reverb;
use alpha_engine::audio::audio_engine::effects::{BaseEffect, Effect, EffectInput};
use alpha_engine::audio::audio_engine::engine::SoundEngine;
use alpha_engine::audio::audio_engine::futures::executor::block_on;
use alpha_engine::audio::audio_engine::math::Matrix4Ext;
use alpha_engine::audio::audio_engine::source::generic::GenericSourceBuilder;
use alpha_engine::audio::audio_engine::source::spatial::SpatialSourceBuilder;
use alpha_engine::audio::audio_engine::source::{SoundSource, Status};
use alpha_engine::event::event_manager::EventManager;
use alpha_engine::event::{self, DeviceEvent, DeviceId, KeyboardInput, VirtualKeyCode};
use alpha_engine::game_objects::game_object::{GmObj, BaseGameObjectProperties};
use alpha_engine::game_objects::implementations::generic_game_object::GenericGameObject;
use alpha_engine::helpers::math::array3_to_vec3;
use alpha_engine::{engine, game, shaders::Shader, sys, text};
use engine::Engine;
use game::Game;
use sys::{
    axes::Axis, cam::projection_ortho::ProjectionOrtho,
    cam::projection_perspective::ProjectionPerspective, fullscreen::Fullscreen,
    system::System,
};
use std::collections::HashMap;

use crate::card::Card;
fn start(system: &mut System, event_manager: &mut EventManager) {
    system.set_window_fullscreen(Fullscreen::False);
    system.set_window_maximized(true);
    system.set_current_shader(Shader::Basic);

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
      #{"Base":"src/sprites/card.png"},
      "Base",
       0);
    add_game_object(game_objects,"script object",generic_object);
    
    */
  "#.to_string());
  system.render_text("Test".to_string(),[300.0,300.0], 200000.0, [100,200,100]);
  system.render_text("Test2".to_string(),[1000.0,300.0], 1000.0, [200,255,0]);

  system.render_text("aaaaaaaaaaa".to_string(),[100.0,300.0], 1000.0, [200,255,0]);

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

fn process_inputs(_system: &mut System, key: KeyboardInput, _device_id: DeviceId) {
    let key_code = key.virtual_keycode;
    match key_code {
        None => println!("Key not recognized"),
        Some(virtual_key) => match virtual_key {
           /*  VirtualKeyCode::N => match key.state {
                alpha_engine::event::ElementState::Pressed => {
                   
                }
                _ => (),
            }, */

            _ => (),
        },
    };
}

fn setup_sound(system: &mut System) {
    let mut sound_context = system.create_sound_context();

    sound_context = system.add_sound_context("Basic context".to_string(), sound_context);
    let sound_buffer = system.add_sound_buffer_from_file(
        "Punch".to_string(),
        "src/audio/punch.wav".to_string(),
        false,
    );

    let generic_source = GenericSourceBuilder::new()
        .with_buffer(sound_buffer)
        .with_gain(0.25)
        .with_status(Status::Paused);
    let source = system.create_sound_source_from_generic(generic_source, false);
    let handle = system.add_source_to_context("Basic context".to_string(), source);
    system.add_sound_source("Punch".to_string(), handle);

    let sine_source = system.add_sound_buffer_from_file(
        "440".to_string(),
        "src/audio/440.wav".to_string(),
        false,
    );

    // Left spatial source
    let generic_left_source = GenericSourceBuilder::new()
        .with_buffer(sine_source.clone())
        .with_status(Status::Paused)
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
        .with_buffer(sine_source.clone())
        .with_status(Status::Paused)
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
        .with_buffer(sine_source)
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
    let reverb_handle = sound_context.state().add_effect(Effect::Reverb(reverb));

    sound_context
        .state()
        .effect_mut(reverb_handle)
        .add_input(EffectInput::direct(handle));
}
fn setup_game_objects(system: &mut System) {

    set_background(system);
    set_characters(system);
}
fn set_characters(system: &mut System){
    let mut sprites = HashMap::new();
    sprites.insert("Idle".to_string(), "src/sprites/Characters/Skeleton/Idle".to_string());
    sprites.insert("Attack".to_string(), "src/sprites/Characters/Skeleton/Attack".to_string());

    let mut skeleton = GenericGameObject::game_object_from_sprites(
        [1500.0, 70.0, 0.0],
        sprites,
        "Idle".to_string(),
        1,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );

    skeleton.base_properties_mut().transform_mut().set_local_scale([4.0, 4.0, 1.0]);

    system.game_objects_mut().add_game_object("Skeleton".to_string(), Box::new(skeleton));



    let mut sprites = HashMap::new();
    sprites.insert("Idle".to_string(), "src/sprites/Characters/Wizard/Idle".to_string());
    sprites.insert("Attack1".to_string(), "src/sprites/Characters/Wizard/Attack1".to_string());
    let mut wizard = GenericGameObject::game_object_from_sprites(
        [200.0, 120.0, 0.0],
        sprites,
        "Idle".to_string(),
        19999,
        true,
        |_system| {
            
        },
        |_system| {
            
        },
        |_system| {
            
        },
        
    );
    wizard.base_properties_mut().transform_mut().set_local_scale([3.0, 3.0, 1.0]);

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
