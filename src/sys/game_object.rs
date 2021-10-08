use glium::Display;

pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;

/**
Struct that represents an object of the game.


*/
#[derive(Debug)]
pub struct GameObject {
    transform: Transform,
}

impl GameObject {
    pub fn new(transform: Transform, mesh: Mesh) -> Self {
        let game_object = Self { transform};

        game_object
    }
    pub fn game_object_from_sprite(transform: Transform)->Self {
        let sprite = GameObject{transform};
        sprite
        
    }
}
