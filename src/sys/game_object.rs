use image::{DynamicImage, GenericImageView};

pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]
pub struct GameObject {
    transform: Transform,
    mesh: Mesh,
    texture: DynamicImage,
}

impl GameObject {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture_path: String) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        let scale_vec3 = glm::vec3(size[0], size[1], size[2]);
        let transform = Transform::new(position_vec3, scale_vec3);
        let texture = image::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => texture,
            Err(_) => image::open("../assets/sprites/default.png").unwrap(),
        };

        let game_object = Self {
            transform,
            mesh,
            texture,
        };

        game_object
    }
    pub fn game_object_from_sprite(position: [f32; 3], texture_path: String) -> Self {
        let mesh = Mesh::create_rectangle();

        let texture = image::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => texture,
            Err(_) => image::open("../assets/sprites/default.png").unwrap(),
        };
        let texture_w = texture.width() as f32;
        let texture_h = texture.height() as f32;

        GameObject::new(position, [texture_w, texture_h, 1.0], mesh, texture_path)
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

    pub fn texture(&self) -> &DynamicImage {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut DynamicImage {
        &mut self.texture
    }
}
