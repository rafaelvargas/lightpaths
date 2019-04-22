use std::ops::{Add, Sub};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn dot_product(v1: Vector, v2: Vector) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross_product(v1: Vector, v2: Vector) -> Vector {
        Vector {
            x: (v1.y * v2.z) - (v1.z * v2.y),
            y: (v1.z * v2.x) - (v1.x * v2.z),
            z: (v1.x * v2.y) - (v1.y * v2.x),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(7.0, 4.0, 3.0);
        assert_eq!(v1 + v2, Vector::new(10.0, 6.0, 4.0));
    }

    #[test]
    fn dot_product_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(7.0, 4.0, 3.0);
        assert_eq!(Vector::dot_product(v1, v2), 32.0);
    }

    #[test]
    fn cross_product_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(7.0, 4.0, 3.0);
        assert_eq!(Vector::cross_product(v1, v2), Vector::new(2.0, -2.0, -2.0));
    }
}