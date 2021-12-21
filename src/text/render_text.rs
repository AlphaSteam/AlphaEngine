#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub position: [f32; 2],
    pub color: [u8; 3],
}

impl Text {
    pub fn new(
        text: String,
        position: [f32; 2],
        scale: f32,
        color: [u8; 3],
    ) -> Self {
        Text {
            text,
            position,
            color,
        }
    }
}
