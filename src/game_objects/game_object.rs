use std::any::Any;
use std::fmt::Debug;
use std::io::Cursor;

pub use crate::rendering::mesh::Mesh;
use crate::sys::system::System;
pub use crate::sys::transform::Transform;
use dyn_clonable::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageBuffer, Rgba, RgbaImage};

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]

pub struct BaseGameObjectProperties {
    transform: Transform,
    mesh: Mesh,
    texture: RgbaImage,
}

impl BaseGameObjectProperties {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture_path: String) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        let scale_vec3 = glm::vec3(size[0], size[1], size[2]);
        let transform = Transform::new(position_vec3, scale_vec3);
        let texture = ImageReader::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => match texture.decode() {
                Ok(image) => image.to_rgba8(),
                Err(_) => image::load(
                    Cursor::new(&include_bytes!("../assets/sprites/default.png")),
                    ImageFormat::Png,
                )
                .unwrap().to_rgba8(),
            },

            Err(_) => image::load(
                Cursor::new(&include_bytes!("../assets/sprites/default.png")),
                ImageFormat::Png,
            )
            .unwrap().to_rgba8(),
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
            Err(_) => {
                image::load_from_memory(include_bytes!("../assets/sprites/default.png")).unwrap()
            }
        };
        let texture_w = texture.width() as f32;
        let texture_h = texture.height() as f32;
        BaseGameObjectProperties::new(position, [texture_w, texture_h, 1.0], mesh, texture_path)
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

    pub fn texture(&self) -> &RgbaImage {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut RgbaImage {
        &mut self.texture
    }
}



#[clonable]
pub trait GmObj: Clone {
    fn base_properties(&self) -> &BaseGameObjectProperties;
    fn base_properties_mut(&mut self) -> &mut BaseGameObjectProperties;
    fn start(&mut self) -> fn(&mut System);
    fn update(&mut self) -> fn(&mut System);
    fn stop(&mut self) -> fn(&mut System);
}

#[clonable]
pub trait GameObject: GmObj + Any + Clone + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T> GameObject for T
where
    T: GmObj + Any + Send + Sync,
{ 
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
