use glm::{Scalar, TVec3};

pub fn array3_to_vec3<T: Copy + Scalar>(arr: [T; 3]) -> TVec3<T> {
    let arr = arr.clone();
    glm::vec3(arr[0], arr[1], arr[2])
}
