use std::{any::Any, path::PathBuf};
use std::fs::read_dir;
use std::io::Error;
use std::fmt::Debug;

pub use crate::rendering::mesh::Mesh;
use crate::rendering::texture::Texture;
use crate::sys::system::System;
pub use crate::sys::transform::Transform;
use dyn_clonable::*;
use image::{RgbaImage};
use sheep::{InputSprite, AmethystFormat, SimplePacker};
use crate::helpers::image::load_texture;

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]

pub struct BaseGameObjectProperties {
    transform: Transform,
    mesh: Mesh,
    texture: Texture,
}

impl BaseGameObjectProperties {
    pub fn new(position: [f32; 3], size: [f32; 3], mesh: Mesh, texture: Texture, z_index: i32) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        let scale_vec3 = glm::vec3(size[0], size[1], size[2]);
        let mut transform = Transform::new(position_vec3, scale_vec3);
        transform.set_z_index(z_index);
      
        let game_object = Self {
            transform,
            mesh,
            texture,
        };

        game_object
    }
    pub fn game_object_from_sprite(position: [f32; 3], texture_path: String, z_index: i32) -> Self {
        let mesh = Mesh::create_rectangle();
        let mut is_sprite_sheet = false;
        let mut sprites: Vec<InputSprite> = Vec::new();
        let mut texture_w=0.0;
        let mut texture_h=0.0;

        let mut texture_file=RgbaImage::new(0,0);
        match read_dir(texture_path.clone()){
            Ok(entries) => {
                is_sprite_sheet = true;
                let mut entries = entries.map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, Error>>().unwrap();
                entries.sort();


                for entry in entries{
                    let file_name = entry.file_stem().unwrap();
                    let folder_name_buf = PathBuf::from(texture_path.clone());
                    let folder_name = folder_name_buf.file_stem().unwrap();
                    println!("Comparison: {:?}, {:?}", file_name, folder_name);
                    if file_name != folder_name {
                        let sprite = load_texture(entry.to_str().unwrap().to_string());
                        texture_w = sprite.width() as f32;
                        texture_h = sprite.height() as f32;
                        let input_sprite = InputSprite{
                            bytes: sprite.as_raw().to_vec(),
                            dimensions: sprite.dimensions()
                        };
                        sprites.push(input_sprite)
                    }
                   
                }



            },
            Err(_) => {
                let sprite = load_texture(texture_path.clone());
                let input_sprite = InputSprite{
                    bytes: sprite.as_raw().to_vec(),
                    dimensions: sprite.dimensions()
                };
                sprites.push(input_sprite);
                texture_w = sprite.width() as f32;
                texture_h = sprite.height() as f32;
                texture_file = sprite;
            },
        }
       
        
        let results = sheep::pack::<SimplePacker>(sprites, 4, Default::default());
        let sprite_sheet = results
        .into_iter()
        .next()
        .expect("Should have returned a spritesheet");
        let meta = sheep::encode::<AmethystFormat>(&sprite_sheet, ());
        // Lastly, we serialize the meta info using serde. This can be any format
        // you want, just implement the trait and pass it to encode.
        //let meta_str = ron::ser::to_string(&meta).expect("Failed to encode meta file");
        let mut save_path=" ".to_string();
        
        if is_sprite_sheet{
            let outbuf = image::RgbaImage::from_vec(
                sprite_sheet.dimensions.0,
                sprite_sheet.dimensions.1,
                sprite_sheet.bytes,
            ).expect("Failed to construct image from sprite sheet bytes");
            let directory_path_buf = PathBuf::from(texture_path.clone());
            let directory_name = directory_path_buf.file_name().unwrap();
            
            let mut tmp_save_path = directory_path_buf.join(directory_name);
            tmp_save_path.set_extension("png");
            save_path = tmp_save_path.to_str().unwrap().to_string().clone();
            println!("Texture path: {:?}",save_path);

            outbuf.save(save_path.clone()).expect("Failed to save image");

            texture_file = outbuf;
        }
     
      /*  if !is_sprite_sheet{
           save_path = texture_path.clone();
       } */
       let texture = Texture::new(texture_file,meta);

        BaseGameObjectProperties::new(position, [texture_w, texture_h, 1.0], mesh, texture, z_index)
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

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut Texture {
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
