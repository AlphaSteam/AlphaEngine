use nalgebra_glm::Vec3;

#[derive(Debug)]
pub enum Axis {
    XAxis,
    YAxis,
    ZAxis,
}

impl Axis {
    pub fn value(&self) -> Vec3 {
        match *self {
            Axis::XAxis => glm::vec3(1.0, 0.0, 0.0),
            Axis::YAxis => glm::vec3(0.0, 1.0, 0.0),
            Axis::ZAxis => glm::vec3(0.0, 0.0, 1.0),
        }
    }
}
