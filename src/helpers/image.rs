use std::io::Cursor;

use image::{RgbaImage, ImageFormat};
use image::io::Reader as ImageReader;


pub fn load_texture(texture_path:  String)-> RgbaImage{
    let texture = ImageReader::open(texture_path.clone());
     match texture {
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
    }
}