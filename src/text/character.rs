use ab_glyph::Rect;
use glium::texture::SrgbTexture2d;

#[derive(Debug)]
pub struct Character {
    pub symbol: char,
    pub texture: SrgbTexture2d,
    pub size: (u32, u32),
    pub bearing: f32,
    pub height: f32,
    pub advance: f32,
    pub bound: Rect,
}

impl Character {
    pub fn new(
        symbol: char,
        texture: SrgbTexture2d,
        size: (u32, u32),
        bearing: f32,
        height: f32,
        advance: f32,
        bound: Rect,
    ) -> Self {
        Character {
            symbol,
            texture,
            size,
            bearing,
            height,
            advance,
            bound,
        }
    }
}
