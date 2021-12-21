use std::collections::HashMap;
use std::{any::Any, path::PathBuf};
use std::fs::read_dir;
use std::io::Error;
use std::fmt::Debug;

use crate::animation::animations::Animations;
pub use crate::rendering::mesh::Mesh;
use crate::rendering::texture::Texture;
use crate::sys::system::System;
pub use crate::sys::transform::Transform;
use dyn_clonable::*;
use image::{RgbaImage};
use sheep::{InputSprite, AmethystFormat, MaxrectsPacker};
use crate::helpers::image::load_texture;

/**
Struct that represents an object of the game.


*/
#[derive(Clone, Debug)]

pub struct BaseGameObjectProperties {
    transform: Transform,
    meshes: HashMap<String, Mesh>,
    animations: Animations,
    should_render: bool,
    ui: bool,
}

impl BaseGameObjectProperties {
    pub fn new(position: [f32; 3],
        size: [f32; 3],
        meshes: HashMap<String, Mesh>,
        animations: Animations,
        z_index: i32,
        should_render: bool,
        ui:bool,
    ) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        let scale_vec3 = glm::vec3(size[0], size[1], size[2]);
        let mut transform = Transform::new(position_vec3, scale_vec3);
        transform.set_z_index(z_index);
      
        let game_object = Self {
            transform,
            meshes,
            animations,
            should_render,
            ui
        };

        game_object
    }

    pub fn game_object_from_sprites(
        position: [f32; 3],
        texture_paths: HashMap<String,String>,
        default_texture: String, 
        z_index: i32,
        should_render: bool,
        ui:bool,
    ) -> Self {
        
        let mut meshes : HashMap<String, Mesh> = HashMap::new();
        let mut textures : HashMap<String, Texture> = HashMap::new();
        
        for (texture_name,texture_path) in texture_paths.iter(){
        let mut sprites: Vec<InputSprite> = Vec::new();
        let mut texture_file=RgbaImage::new(0,0);
        let mut is_sprite_sheet = false;
        let mut texture_w=0.0;
        let mut texture_h=0.0;

        match read_dir(texture_path.clone()){
            Ok(entries) => {
                is_sprite_sheet = true;
                let mut entries = entries.map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, Error>>().unwrap();
                entries.sort_by(|name_a, name_b|{
                    Ord::cmp(name_a,name_b).reverse()

                });


                for entry in entries{
                    let file_name = entry.file_stem().unwrap();

                    let folder_name_buf = PathBuf::from(texture_path.clone());
                    let folder_name = folder_name_buf.file_stem().unwrap();
                    if file_name != folder_name {
                        let sprite = load_texture(entry.to_str().unwrap().to_string());
                        
                        let input_sprite = InputSprite{
                            bytes: sprite.as_raw().to_vec(),
                            dimensions: sprite.dimensions()
                        };
                        sprites.push(input_sprite);
                        texture_w = sprite.width() as f32;
                        texture_h = sprite.height() as f32;
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
        let results = sheep::pack::<MaxrectsPacker>(sprites, 4, Default::default());
        let sprite_sheet = results
        .into_iter()
        .next()
        .expect("Should've returned a spritesheet");
        let meta = sheep::encode::<AmethystFormat>(&sprite_sheet, ());

        let mut _save_path=" ".to_string();
        
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
            _save_path = tmp_save_path.to_str().unwrap().to_string().clone();

            outbuf.save(_save_path).expect("Failed to save image");

            texture_file = outbuf;
        }
     
     
       let mesh = Mesh::create_rectangle_animated(&meta,0);

       let texture = Texture::new(texture_file,(texture_w, texture_h),meta);

       meshes.insert(texture_name.clone(), mesh);
       textures.insert(texture_name.clone(), texture);
        }
        
        let default_texture_obj = textures[&default_texture].clone();
        let texture_w = default_texture_obj.individual_sprite_size().0 as f32;
        let texture_h = default_texture_obj.individual_sprite_size().1 as f32;

        let mut animations = Animations::new(textures);
        *animations.current_animation_mut() = default_texture;
        BaseGameObjectProperties::new(position, [texture_w, texture_h, 1.0], meshes, animations, z_index,should_render, ui)
    }
   
    pub fn meshes(&self) -> &HashMap<String, Mesh> {
        &self.meshes
    }

    pub fn meshes_mut(&mut self) -> &mut HashMap<String, Mesh> {
        &mut self.meshes
    }
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn animations(&self) -> &Animations {
        &self.animations
    }

    pub fn animations_mut(&mut self) -> &mut Animations {
        &mut self.animations
    }
      /**
    Get the render flag. It determines if the mesh should be rendered or not.
    */
    pub fn should_render(&self)->bool{
        self.should_render
    }
     /**
    Set the render flag. It determines if the mesh should be rendered or not.
    */
    pub fn set_should_render(&mut self, val: bool){
        self.should_render = val;
    }

    /**
    Get ui flag. It determines if the texture will appear on a ui element or not.
    */
    pub fn ui(&self)->bool{
        self.ui
    }
      /**
    Set ui flag. It determines if the texture will appear on a ui element or not.
    */
    pub fn set_ui(&mut self, val: bool){
        self.ui = val;
    }
}



#[clonable]
pub trait GmObj: Clone {
    fn base_properties(&self) -> &BaseGameObjectProperties;
    fn base_properties_mut(&mut self) -> &mut BaseGameObjectProperties;
    fn start(&mut self) -> fn(&mut System);
    fn update(&mut self) -> fn(&mut System);
    fn stop(&mut self) -> fn(&mut System);
    fn action(&mut self) -> fn(&mut System);
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
