use std::any::Any;
use std::fmt::Debug;
use std::io::Cursor;

pub use crate::rendering::mesh::Mesh;
pub use crate::sys::transform::Transform;
use dyn_clonable::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use rhai::Array;
/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]

pub struct BaseGameObjectProperties {
    transform: Transform,
    mesh: Mesh,
    texture: DynamicImage,
}

impl BaseGameObjectProperties {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture_path: String) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        let scale_vec3 = glm::vec3(size[0], size[1], size[2]);
        let transform = Transform::new(position_vec3, scale_vec3);
        let texture = ImageReader::open(texture_path.clone());
        let texture = match texture {
            Ok(texture) => match texture.decode() {
                Ok(image) => image,
                Err(_) => image::load(
                    Cursor::new(&include_bytes!("../assets/sprites/default.png")),
                    ImageFormat::Png,
                )
                .unwrap(),
            },

            Err(_) => image::load(
                Cursor::new(&include_bytes!("../assets/sprites/default.png")),
                ImageFormat::Png,
            )
            .unwrap(),
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

    pub fn texture(&self) -> &DynamicImage {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut DynamicImage {
        &mut self.texture
    }
}



#[clonable]
pub trait GmObj: Clone {
    fn get_base_properties(&self) -> &BaseGameObjectProperties;
    fn get_base_properties_mut(&mut self) -> &mut BaseGameObjectProperties;
}

#[clonable]
pub trait GameObject: GmObj + Any + Clone + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T> GameObject for T
where
    T: GmObj + Any + Debug + Send + Sync,
{ 
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct GenericGameObject {
    base_properties: BaseGameObjectProperties,
}
impl GenericGameObject {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture_path: String) -> Self {
        let base_properties = BaseGameObjectProperties::new(position, size, mesh, texture_path);
        GenericGameObject { base_properties }
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
        GenericGameObject::new(position, [texture_w, texture_h, 1.0], mesh, texture_path)
    }
    pub fn game_object_from_sprite_script(position: Array, texture_path: &str) -> Self {
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
        GenericGameObject::new([position[0].clone_cast::<f64>() as f32,position[1].clone_cast::<f64>() as f32,position[2].clone_cast::<f64>() as f32],
         [texture_w, texture_h, 1.0], mesh, texture_path.to_string())
    }
}
impl GmObj for GenericGameObject {
    fn get_base_properties(&self) -> &BaseGameObjectProperties {
        &self.base_properties
    }
    fn get_base_properties_mut(&mut self) -> &mut BaseGameObjectProperties {
        &mut self.base_properties
    }
}
