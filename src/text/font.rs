use std::collections::HashMap;

use super::character::Character;
use ab_glyph::Glyph;
use ab_glyph::{point, Font as abFont, FontRef, ScaleFont};
use glium::{texture::SrgbTexture2d, Display};
use image::{EncodableLayout, Rgb, RgbImage};
#[derive(Debug)]
pub struct Font {
    characters: HashMap<char, Character>,
}
impl Font {
    pub fn new(font_path: &str, display: &Display) -> Self {
        let mut characters = HashMap::new();

        let font_path = font_path.to_string();
        let data = &std::fs::read(font_path).unwrap();
        let font = FontRef::try_from_slice(data).unwrap();
        let scale = 24.0;
        for c in 65_u8..128_u8 {
            // Get glyph
            let glyph_id = font.glyph_id(c as char);
            let q_glyph: Glyph = glyph_id.with_scale_and_position(scale, point(0.0, 0.0));

            // Draw it.
            if let Some(q) = font.outline_glyph(q_glyph) {
                let w = q.px_bounds().width() as u32;
                let h = q.px_bounds().height() as u32;
                let mut img = RgbImage::new(w, h);

                q.draw(|x, y, co| {
                    let value = (255.0 * co) as u8;
                    img.put_pixel(x, y, Rgb([value, value, value]));
                });
                let image = glium::texture::RawImage2d::from_raw_rgb_reversed(
                    &img.as_bytes(),
                    img.dimensions(),
                );

                let texture = SrgbTexture2d::new(display, image).unwrap();
                let h_bearing = font.as_scaled(scale).h_side_bearing(glyph_id);
                let _advance = font.as_scaled(scale).h_advance(glyph_id);
                let character =
                    Character::new(c as char, texture, img.dimensions(), [h_bearing, h_bearing]);
                characters.insert(c as char, character);

                //let path = "./chars/".to_string() + &c.to_string() + &".bmp".to_string();
                //img.save_with_format(path, image::ImageFormat::Bmp).unwrap();
            }
        }

        Font { characters }
    }

    pub fn characters(&self) -> &HashMap<char, Character> {
        &self.characters
    }
}
