use super::math;
use std::ops::{Add, AddAssign, Div};

pub struct Ray {
    pub origin: math::Vector,
    pub direction: math::Vector,
}

impl Ray {
    pub fn new(origin: math::Vector, direction: math::Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

pub struct Color {
    pub r: f32, // Should have values between [0.0, 1.0]
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn clamp(&mut self) {
        if self.r > 1.0 {
            self.r = 1.0;
        }
        if self.g > 1.0 {
            self.g = 1.0;
        }
        if self.b > 1.0 {
            self.b = 1.0;
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<u32> for Color {
    type Output = Color;

    fn div(self, scalar: u32) -> Color {
        Color {
            r: self.r / (scalar as f32),
            g: self.g / (scalar as f32),
            b: self.b / (scalar as f32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
