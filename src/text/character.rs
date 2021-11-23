use glium::texture::SrgbTexture2d;

#[derive(Debug)]
pub struct Character {
    pub symbol: char,
    pub texture: SrgbTexture2d,
    pub size: (u32, u32),
    pub bearing: [f32; 2],
}

impl Character {
    pub fn new(symbol: char, texture: SrgbTexture2d, size: (u32, u32), bearing: [f32; 2]) -> Self {
        Character {
            symbol,
            texture,
            size,
            bearing,
        }
    }
}
