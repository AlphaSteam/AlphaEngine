pub use crate::sys::system::System;
use glm::{identity, Quat};
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Vec3};

/**
Struct in charge of managing an object's local position in the world.
*/

#[derive(Copy, Clone)]
pub struct Transform {
    local_translation: Vec3,
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
    let transform = Transform::new(glm::vec3(0.0, 0.0, 0.0), glm::quat(0.0, 0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0));
     ```
    */
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let component = Self {
            local_translation: translation,
            local_rotation: rotation,
            local_scale: scale,
        };

        component
    }

    /**
    Inmutable access to local translation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    assert_eq!(*transform.local_translation(), glm::vec3(1.0, 0.0, 0.0));
     ```
    */
    pub fn local_translation(&self) -> &Vec3 {
        &self.local_translation
    }

    /**
    Mutable access to local translation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_translation_mut = transform.local_translation_mut();
    *local_translation_mut = glm::vec3(0.0, 0.0, 0.0);
    assert_eq!(*transform.local_translation(), glm::vec3(0.0, 0.0, 0.0));
     ```
    */
    pub fn local_translation_mut(&mut self) -> &mut Vec3 {
        &mut self.local_translation
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
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_rotation_mut = transform.local_rotation_mut();
    *local_rotation_mut = glm::quat(1.0, 1.0, 1.0, 1.0);
    assert_eq!(*transform.local_rotation(), glm::quat(1.0, 1.0, 1.0, 1.0));
     ```
    */
    pub fn local_rotation_mut(&mut self) -> &mut Quat {
        &mut self.local_rotation
    }

    /**
    Inmutable access to local scale.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
    let transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
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
    let mut transform = Transform::new(glm::vec3(1.0, 0.0, 0.0), glm::quat(0.0, 2.0, 0.0, 0.0), glm::vec3(2.0, 0.0, 0.0));
    let mut local_scale_mut = transform.local_scale_mut();
    *local_scale_mut = glm::vec3(0.0, 1.0, 1.0);
    assert_eq!(*transform.local_scale(), glm::vec3(0.0, 1.0, 1.0));
     ```
    */
    pub fn local_scale_mut(&mut self) -> &mut Vec3 {
        &mut self.local_scale
    }

    /**
    Returns model matrix from local translation, scale and rotation.

    # Example
     ```
    # pub use alpha_engine::sys::transform::Transform;
    # use nalgebra_glm as glm;
     ```
    */
    pub fn get_model_matrix(&self) -> Mat4 {
        let translation_matrix = glm::translate(&identity(), &self.local_translation);
        let rotation_matrix = glm::quat_to_mat4(&self.local_rotation);
        let scale_matrix = glm::scale(&glm::identity(), &self.local_scale);

        let model_matrix = translation_matrix * rotation_matrix * scale_matrix;
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
        glm::quat_rotate_vec3(&self.local_rotation, &glm::vec3(0.0, 0.0, 1.0))
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
        glm::quat_rotate_vec3(&self.local_rotation, &glm::vec3(0.0, 1.0, 0.0))
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

        glm::look_at(
            &self.local_translation,
            &(&self.local_translation + forward),
            &up,
        )
    }
}
