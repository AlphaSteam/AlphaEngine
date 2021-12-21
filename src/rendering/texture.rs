use image::RgbaImage;
use sheep::SerializedSpriteSheet;


#[derive(Clone, Debug)]
pub struct Texture{
    meta_data: SerializedSpriteSheet,
    texture:RgbaImage,
    individual_sprite_size: (f32,f32),
}
impl Texture{
    pub fn new(
        texture: RgbaImage,
        individual_sprite_size: (f32,f32),
        meta_data: SerializedSpriteSheet
    )->Self{

        Texture{
            texture,
            meta_data,
            individual_sprite_size
        }
    }
    pub fn meta_data(&self)->&SerializedSpriteSheet{
        &self.meta_data
    }
    pub fn meta_data_mut(&mut self)->&mut SerializedSpriteSheet{
        &mut self.meta_data
    }


    pub fn texture(&self)->&RgbaImage{
        &self.texture
    }

    pub fn texture_mut(&mut self)->&mut RgbaImage{
        &mut self.texture
    }
    pub fn individual_sprite_size(&self) ->&(f32,f32){
        &self.individual_sprite_size
    }
    pub fn individual_sprite_size_mut(&mut self) ->&mut (f32,f32){
        &mut self.individual_sprite_size
    }
}