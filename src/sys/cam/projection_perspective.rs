#[derive(Debug)]
pub struct ProjectionPerspective {
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}
impl ProjectionPerspective {
    pub fn new(aspect: f32, fov: f32, near: f32, far: f32) -> Box<Self> {
        Box::new(ProjectionPerspective {
            aspect,
            fov,
            near,
            far,
        })
    }
}
