#[derive(Clone, Debug)]
pub struct Text {
    pub text: String,
    pub font: String,
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 3],
}

impl Text {
    pub fn new(
        text: String,
        font: String,
        position: [f32; 2],
        scale: [f32; 2],
        color: [f32; 3],
    ) -> Self {
        Text {
            text,
            font,
            position,
            scale,
            color,
        }
    }
}
