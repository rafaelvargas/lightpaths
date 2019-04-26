use super::math;

pub struct Light {
    pub position: math::Vector,
    pub intensity: f32,
}

impl Light {
    pub fn new(p: math::Vector, i: f32) -> Light {
        Light {
            position: p,
            intensity: i,
        }
    }
}
