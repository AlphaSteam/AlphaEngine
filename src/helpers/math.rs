use glm::{Scalar, TVec3};
use nalgebra_glm::RealNumber;

pub fn array3_to_vec3<T: Copy + Scalar>(arr: [T; 3]) -> TVec3<T> {
    let arr = arr.clone();
    glm::vec3(arr[0], arr[1], arr[2])
}

pub fn normalize_number<T: RealNumber>(val: T , min: T, max: T)-> T { 
    (val - min) / (max - min)
}