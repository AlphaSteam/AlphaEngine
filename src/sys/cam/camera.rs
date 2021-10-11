use crate::sys::game_object::Transform;

use super::projection::Projection;

#[derive(Debug)]
pub struct Camera {
    transform: Transform,
    projection: Box<dyn Projection>,
}
impl Camera {
    pub fn new(position: [f32; 3], look_at: [f32; 3], projection: Box<dyn Projection>) -> Self {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);

        let mut transform = Transform::new(position_vec3, glm::vec3(1.0, 1.0, 1.0));
        let local_rotation = transform.local_rotation_mut();

        let look_at_vec3 = glm::vec3(look_at[0], look_at[1], look_at[2]);
        *local_rotation = glm::quat_look_at(&look_at_vec3, &glm::vec3(0.0, 1.0, 0.0));
        Camera {
            transform,
            projection,
        }
    }
    pub fn set_projection(&mut self, projection: Box<dyn Projection>) {
        self.projection = projection;
    }
    pub fn projection(&self) -> &Box<dyn Projection> {
        &self.projection
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
    pub fn look_at(&mut self, direction: [f32; 3]) {
        let local_rotation = self.transform.local_rotation_mut();
        let direction_vec3 = glm::vec3(direction[0], direction[1], direction[2]);
        *local_rotation = glm::quat_look_at(&direction_vec3, &glm::vec3(0.0, 1.0, 0.0));
    }
}
