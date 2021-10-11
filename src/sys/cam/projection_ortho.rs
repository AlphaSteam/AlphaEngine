#[derive(Debug)]
pub struct ProjectionOrtho {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl ProjectionOrtho {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, znear: f32, zfar: f32) -> Box<Self> {
        Box::new(ProjectionOrtho {
            left,
            right,
            bottom,
            top,
            znear,
            zfar,
        })
    }
}
