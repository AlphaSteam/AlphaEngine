pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]
pub struct GameObject {
    transform: Transform,
    mesh: Mesh,
    texture_path: String,
}

impl GameObject {
    pub fn new(transform: Transform, mesh: Mesh, texture_path: String) -> Self {
        let game_object = Self {
            transform,
            mesh,
            texture_path,
        };

        game_object
    }
    pub fn game_object_from_sprite(transform: Transform, texture_path: String) -> Self {
        let mesh = Mesh::create_rectangle();
        let sprite = GameObject {
            transform,
            mesh,
            texture_path,
        };
        sprite
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn texture_path(&self) -> &String {
        &self.texture_path
    }

    pub fn texture_path_mut(&mut self) -> &mut String {
        &mut self.texture_path
    }
}
