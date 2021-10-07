pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;

/**
Struct that represents an object of the game.


*/

pub struct GameObject<'a> {
    transform: Transform,
    mesh: Mesh<'a>,
    Texture: Texture,
}

impl GameObject {
    pub fn new(transform: Transform, mesh: Mesh) -> Self {
        let game_object = Self { transform, mesh };

        game_object
    }
}
