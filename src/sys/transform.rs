use crate::sys::axes::Axis;
pub use crate::sys::private_system::PrivateSystem;
use glm::{identity, Quat};
use glm::{Mat4, Vec3};
/**
Struct in charge of managing an object's local position in the world.
*/

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    local_position: Vec3,
    local_rotation: Quat,
    local_scale: Vec3,
}

impl Transform {
    /**
    Initializes transform component.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0));
     ```
    */
    pub fn new(position: Vec3, scale: Vec3) -> Self {
        let component = Self {
            local_position: position,
            local_rotation: glm::quat_identity(),
            local_scale: scale,
        };

        component
    }

    /**
    Inmutable access to local position.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
     ```
    */
    pub fn local_position(&self) -> &Vec3 {
        &self.local_position
    }

    /**
    Mutable access to local position.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_position_mut = transform.local_position_mut();
    *local_position_mut = glm::vec3(0.0, 0.0, 0.0);
     ```
    */
    pub fn local_position_mut(&mut self) -> &mut Vec3 {
        &mut self.local_position
    }

    pub fn translate(&mut self, position: [f32; 3]) {
        let position_vec3 = glm::vec3(position[0], position[1], position[2]);
        self.local_position = self.local_position + position_vec3;
    }

    /**
    Inmutable local rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    assert_eq!(*transform.local_rotation(), glm::quat(0.0, 2.0, 0.0, 0.0));
     ```
    */
    pub fn local_rotation(&self) -> &Quat {
        &self.local_rotation
    }

    /**
    Mutable local rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_rotation_mut = transform.local_rotation_mut();
    *local_rotation_mut = glm::quat(1.0, 1.0, 1.0, 1.0);
    assert_eq!(*transform.local_rotation(), glm::quat(1.0, 1.0, 1.0, 1.0));
     ```
    */
    pub fn local_rotation_mut(&mut self) -> &mut Quat {
        &mut self.local_rotation
    }

    pub fn rotate(&mut self, axis: Axis, angle: f32) {
        let axis_value = axis.value();
        self.local_rotation = glm::quat_rotate(&self.local_rotation, angle, &axis_value);
    }

    pub fn scale(&mut self, scale: [f32; 3]) {
        let scale_vec3 = glm::vec3(scale[0], scale[1], scale[2]);
        self.local_scale = glm::matrix_comp_mult(&self.local_scale, &scale_vec3);
    }

    /**
    Inmutable access to local scale.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    assert_eq!(*transform.local_scale(), glm::vec3(2.0, 0.0, 0.0));
     ```
    */
    pub fn local_scale(&self) -> &Vec3 {
        &self.local_scale
    }

    /**
    Mutable access to local scale.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_scale_mut = transform.local_scale_mut();
    *local_scale_mut = glm::vec3(0.0, 1.0, 1.0);
     ```
    */
    pub fn local_scale_mut(&mut self) -> &mut Vec3 {
        &mut self.local_scale
    }

    /**
    Returns model matrix from local position, scale and rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_model_matrix(&self) -> Mat4 {
        let mut model_matrix = identity();
        model_matrix = glm::translate(&model_matrix, &self.local_position);
        model_matrix = glm::translate(
            &model_matrix,
            &glm::vec3(
                0.5 * self.local_scale()[0],
                0.5 * self.local_scale()[1],
                0.5 * self.local_scale()[2],
            ),
        );
        model_matrix = model_matrix * glm::quat_cast(&self.local_rotation);
        model_matrix = glm::translate(
            &model_matrix,
            &glm::vec3(
                -0.5 * self.local_scale()[0],
                -0.5 * self.local_scale()[1],
                -0.5 * self.local_scale()[2],
            ),
        );
        model_matrix = glm::scale(&model_matrix, &self.local_scale);
        model_matrix
    }
    /**
    Returns the up vector using the quaternion in local_rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_up_vector(&self) -> Vec3 {
        glm::quat_rotate_vec3(&self.local_rotation, &glm::vec3(0.0, 1.0, 0.0))
    }

    /**
    Returns the forward vector using the quaternion in local_rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_forward_vector(&self) -> Vec3 {
        let forward = glm::quat_rotate_vec3(&self.local_rotation, &glm::vec3(0.0, 0.0, 1.0));
        forward
    }

    /**
    Returns the right vector using the quaternion in local_rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_right_vector(&self) -> Vec3 {
        glm::quat_rotate_vec3(&self.local_rotation, &glm::vec3(1.0, 0.0, 0.0))
    }

    /**
    Returns local scale.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_view_matrix(&self) -> Mat4 {
        let up = self.get_up_vector();
        let forward = self.get_forward_vector();
        let look_at = glm::look_at(&self.local_position, &(self.local_position + forward), &up);
        look_at
    }
}
