use std::collections::HashMap;

use ab_glyph::{point, Font as abFont, FontArc, FontRef, Glyph, ScaleFont};
use glium::{texture::SrgbTexture2d, Display};
use glyph_brush::{BrushAction, BrushError, GlyphBrushBuilder, Rectangle, Section, Text};
use image::{EncodableLayout, ImageBuffer, Rgb, RgbImage};

use super::character::Character;
#[derive(Debug)]
pub struct Font {
    characters: HashMap<char, Character>,
}
impl Font {
    pub fn new(font_path: &str, display: &Display) -> Self {
        let mut characters = HashMap::new();

        let dejavu = FontArc::try_from_slice(include_bytes!("/home/alphasteam/Escritorio/Dev/Motor de videojuegos/alpha_engine/src/fonts/ArialCE.ttf")).unwrap();
        let mut glyph_brush = GlyphBrushBuilder::using_font(dejavu).build();
        let scale = 100.0;
        for c in 0..125 as u8 {
            // Get glyph
            //let glyph_id = font.glyph_id(c as char);
            //let q_glyph: Glyph = glyph_id.with_scale_and_position(scale, point(0.0, 0.0));

            glyph_brush.queue(Section::default().add_text(Text::new((c.to_string()).as_str())));

            match glyph_brush.process_queued(
                |rect, tex_data| Self::update_texture(rect, tex_data, c as char),
                |vertex_data| println!("{:?}", vertex_data),
            ) {
                Ok(BrushAction::Draw(vertices)) => {
                    // Draw new vertices.
                }
                Ok(BrushAction::ReDraw) => {
                    // Re-draw last frame's vertices unmodified.
                }
                Err(BrushError::TextureTooSmall { suggested }) => {
                    // Enlarge texture + glyph_brush texture cache and retry.
                }
            }
        }

        Font { characters }
    }
    fn update_texture(rect: Rectangle<u32>, tex_data: &[u8], c: char) {}
    pub fn characters(&self) -> &HashMap<char, Character> {
        &self.characters
    }
}
