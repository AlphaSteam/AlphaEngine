use image::RgbaImage;
use sheep::SerializedSpriteSheet;


#[derive(Clone, Debug)]
pub struct Texture{
    meta_data: SerializedSpriteSheet,
    texture:RgbaImage,
}
impl Texture{
    pub fn new(texture: RgbaImage, meta_data: SerializedSpriteSheet)->Self{

        Texture{texture,meta_data}
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
}