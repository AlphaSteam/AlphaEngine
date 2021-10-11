use nalgebra_glm::TMat4;

use super::{projection_ortho::ProjectionOrtho, projection_perspective::ProjectionPerspective};

pub trait Projection: std::fmt::Debug {
    fn get_projection(&self) -> TMat4<f32>;
}

impl Projection for ProjectionOrtho {
    fn get_projection(&self) -> TMat4<f32> {
        let ortho = glm::ortho(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.znear,
            self.zfar,
        );
        ortho
    }
}
impl Projection for ProjectionPerspective {
    fn get_projection(&self) -> TMat4<f32> {
        glm::perspective(self.aspect, self.fov, self.near, self.far)
    }
}
