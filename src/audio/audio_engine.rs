//! Module in charge of the audio processing of the engine.
pub use rg3d_sound::*;
use std::{collections::HashMap, sync::Arc};

use rg3d_sound::{
    algebra::UnitQuaternion,
    buffer::{DataSource, SoundBufferResource},
    context::SoundContext,
    engine::SoundEngine,
    futures::executor::block_on,
    math::Matrix4Ext,
    pool::Handle,
    source::{generic::GenericSourceBuilder, spatial::SpatialSourceBuilder, SoundSource},
};

use crate::sys::cam::camera::Camera;

/// Struct that has the sound engine and it's resources.
pub struct AudioEngine {
    sound_engine: Arc<std::sync::Mutex<SoundEngine>>,
    sound_contexts: HashMap<String, SoundContext>,
    sound_buffers: HashMap<String, SoundBufferResource>,
    sound_sources: HashMap<String, Handle<SoundSource>>,
}
impl AudioEngine {
    /// AudioEngine initializer.
    pub fn new() -> Self {
        AudioEngine {
            sound_contexts: HashMap::new(),
            sound_buffers: HashMap::new(),
            sound_sources: HashMap::new(),
            sound_engine: SoundEngine::new(),
        }
    }
    /// Returns inmutable reference to sound contexts Hashmap.
    pub fn sound_contexts(&self) -> &HashMap<String, SoundContext> {
        &self.sound_contexts
    }
    /// Returns mutable reference to sound contexts Hashmap.
    pub fn sound_contexts_mut(&mut self) -> &mut HashMap<String, SoundContext> {
        &mut self.sound_contexts
    }

    /// Creates sound context and sets listener basis to a camera.
    pub fn create_sound_context(&self, camera: &Camera) -> SoundContext {
        let sound_context = SoundContext::new();
        let mut context = sound_context.state();
        let listener = context.listener_mut();
        let rotation_matrix = camera.transform().local_rotation();
        let basis = UnitQuaternion::from_quaternion(*rotation_matrix)
            .to_homogeneous()
            .basis();
        listener.set_basis(basis);
        sound_context.clone()
    }
    /// Returns mutable reference to sound context of given id.

    pub fn get_sound_context_mut(&mut self, sound_context_id: String) -> Option<&mut SoundContext> {
        let entry = self.sound_contexts.entry(sound_context_id);
        match entry {
            std::collections::hash_map::Entry::Occupied(object) => Some(object.into_mut()),
            std::collections::hash_map::Entry::Vacant(_) => None,
        }
    }
    /// Returns inmutable reference to sound context of given id.

    pub fn get_sound_context(&self, sound_context_id: String) -> Option<&SoundContext> {
        let entry = self.sound_contexts.get_key_value(&sound_context_id);
        match entry {
            Some((_, sound_context)) => Some(sound_context),
            None => todo!(),
        }
    }

    /// Registers sound context in sound engine and adds it to system HashMap.
    pub fn add_sound_context(
        &mut self,
        sound_context_id: String,
        sound_context: SoundContext,
    ) -> SoundContext {
        let sound_contexts = self.sound_contexts_mut();
        sound_contexts
            .entry(sound_context_id)
            .or_insert(sound_context.clone());
        self.sound_engine
            .lock()
            .unwrap()
            .add_context(sound_context.clone());
        sound_context
    }

    /// Removes sound context from system HashMap.
    pub fn remove_sound_context(&mut self, sound_context_id: String) -> Option<SoundContext> {
        let sound_contexts = self.sound_contexts_mut();
        sound_contexts.remove(&sound_context_id)
    }
    /// Returns reference to sound engine.

    pub fn sound_engine(&self) -> &Arc<std::sync::Mutex<SoundEngine>> {
        &self.sound_engine
    }
    /// Returns inmutable reference to sound buffers HashMap.

    pub fn sound_buffers(&self) -> &HashMap<String, SoundBufferResource> {
        &self.sound_buffers
    }
    /// Returns mutable reference to sound buffers HashMap.

    pub fn sound_buffers_mut(&mut self) -> &mut HashMap<String, SoundBufferResource> {
        &mut self.sound_buffers
    }
    /// Creates sound buffer from a file path and adds it to system HashMap.

    pub fn add_sound_buffer_from_file(
        &mut self,
        sound_buffer_id: String,
        sound_path: String,
        stream: bool,
    ) -> SoundBufferResource {
        let sound_buffers = self.sound_buffers_mut();
        let new_sound_buffer;
        if stream {
            new_sound_buffer = SoundBufferResource::new_streaming(
                block_on(DataSource::from_file(sound_path)).unwrap(),
            )
            .unwrap();
        } else {
            new_sound_buffer = SoundBufferResource::new_generic(
                block_on(DataSource::from_file(sound_path)).unwrap(),
            )
            .unwrap();
        }
        sound_buffers
            .entry(sound_buffer_id)
            .or_insert(new_sound_buffer.clone());

        new_sound_buffer
    }
    pub fn remove_sound_buffer(&mut self, sound_buffer_id: String) -> Option<SoundBufferResource> {
        let sound_buffers = self.sound_buffers_mut();
        sound_buffers.remove(&sound_buffer_id)
    }
    pub fn sound_sources(&self) -> &HashMap<String, Handle<SoundSource>> {
        &self.sound_sources
    }

    pub fn sound_sources_mut(&mut self) -> &mut HashMap<String, Handle<SoundSource>> {
        &mut self.sound_sources
    }
    pub fn create_sound_source_from_generic(
        &self,
        generic_source_builder: GenericSourceBuilder,
        spatial: bool,
    ) -> SoundSource {
        if spatial {
            SpatialSourceBuilder::new(generic_source_builder.build().unwrap()).build_source()
        } else {
            generic_source_builder.build_source().unwrap()
        }
    }
    pub fn add_sound_source(
        &mut self,
        sound_source_id: String,
        sound_source: Handle<SoundSource>,
    ) -> Handle<SoundSource> {
        let sound_sources = self.sound_sources_mut();

        sound_sources.entry(sound_source_id).or_insert(sound_source);

        sound_source
    }
    pub fn get_sound_source(&self, sound_source_id: String) -> Option<&Handle<SoundSource>> {
        self.sound_sources().get(&sound_source_id)
    }

    pub fn get_sound_source_mut(
        &mut self,
        sound_source_id: String,
    ) -> Option<&mut Handle<SoundSource>> {
        self.sound_sources_mut().get_mut(&sound_source_id)
    }
    pub fn remove_sound_source(&mut self, sound_source_id: String) -> Option<Handle<SoundSource>> {
        let sound_sources = self.sound_sources_mut();
        sound_sources.remove(&sound_source_id)
    }
    pub fn add_source_to_context(
        &mut self,
        sound_context_id: String,
        source: SoundSource,
    ) -> Handle<SoundSource> {
        self.get_sound_context_mut(sound_context_id)
            .unwrap()
            .state()
            .add_source(source)
    }
}
