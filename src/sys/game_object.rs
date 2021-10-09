use glium::Display;

pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]
pub struct GameObject {
    transform: Transform,
    mesh: Mesh,
}

impl GameObject {
    pub fn new(transform: Transform, mesh: Mesh) -> Self {
        let game_object = Self { transform, mesh };

        game_object
    }
    pub fn game_object_from_sprite(transform: Transform) -> Self {
        let mesh = Mesh::create_rectangle();
        let sprite = GameObject { transform, mesh };
        sprite
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
