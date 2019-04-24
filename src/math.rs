use std::ops::{Add, Div, Mul, Neg, Sub};

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

    pub fn normalize(self) -> Vector {
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
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

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_vectors_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(7.0, 4.0, 3.0);
        assert_eq!(v1 + v2, Vector::new(10.0, 6.0, 4.0));
    }

    #[test]
    fn subtraction_of_vectors_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(7.0, 4.0, 3.0);
        assert_eq!(v1 - v2, Vector::new(-4.0, -2.0, -2.0));
    }

    #[test]
    fn multiplication_vector_by_scalar_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        assert_eq!(v1 * 3.0, Vector::new(9.0, 6.0, 3.0));
    }

    #[test]
    fn division_vector_by_scalar_test() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        assert_eq!(v1 / 2.0, Vector::new(1.5, 1.0, 0.5));
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

    #[test]
    fn normalization_of_vector_test() {
        let v1 = Vector::new(2.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn magnitude_of_vector() {
        let v1 = Vector::new(2.0, 2.0, 1.0);
        assert_eq!(v1.magnitude(), 3.0);
    }
}
