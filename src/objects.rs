use super::math;

pub struct Sphere {
    pub center: math::Vector,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: math::Vector, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}
