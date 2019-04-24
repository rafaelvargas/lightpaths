use super::math;

pub struct Ray {
    pub origin: math::Vector,
    pub direction: math::Vector,
}

impl Ray {
    fn new(origin: math::Vector, direction: math::Vector) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Camera {
    position: math::Vector,
    focal_distance: f32,
    lens_dimensions: Dimensions,
    screen_dimensions: Dimensions,
    lens_top_left_corner: math::Vector,
    u: math::Vector,
    v: math::Vector,
    w: math::Vector,
}

impl Camera {
    pub fn new(
        position: math::Vector,
        direction: math::Vector,
        focal_distance: f32,
        lens_dimensions: Dimensions,
        screen_dimensions: Dimensions,
    ) -> Camera {
        let up_vector = math::Vector::new(0.0, 1.0, 0.0);
        let w = -direction.normalize();
        let u = math::Vector::cross_product(up_vector, w).normalize();
        let v = math::Vector::cross_product(w, u);

        Camera {
            position: position,
            focal_distance: focal_distance,
            lens_dimensions: lens_dimensions,
            screen_dimensions: screen_dimensions,
            lens_top_left_corner: position + v * (lens_dimensions.height / 2.0)
                - u * (lens_dimensions.width / 2.0),
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn generate_ray(&self, i: f32, j: f32) -> Ray {
        let ray_direction = self.lens_top_left_corner
            - self.v
                * self.lens_dimensions.height
                * ((i as f32 + 0.5) / self.screen_dimensions.height)
            + self.u
                * self.lens_dimensions.width
                * ((j as f32 + 0.5) / self.screen_dimensions.width)
            - self.w * self.focal_distance;

        Ray::new(self.position, ray_direction)
    }
}

pub struct Light {
    position: math::Vector,
    intensity: f32,
}

impl Light {
    pub fn new(p: math::Vector, i: f32) -> Light {
        Light {
            position: p,
            intensity: i,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dimensions {
    pub height: f32,
    pub width: f32,
}

impl Dimensions {
    pub fn new(height: f32, width: f32) -> Dimensions {
        Dimensions {
            height: height as f32,
            width: width as f32,
        }
    }
}

/*
    TODO:
        - Material
        - Color
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_coordinates_test() {
        let position = math::Vector::new(0.0, 0.0, 0.0);
        let direction = math::Vector::new(0.0, 0.0, 1.0);
        let camera = util::Camera::new(
            position,
            direction,
            1.0,
            Dimensions::new(2.0, 2.0),
            Dimensions::new(20.0, 20.0),
        );

        assert_eq!(
            camera,
            Camera {
                position: math::Vector::new(0.0, 0.0, 0.0),
                focal_distance: 1.0,
                lens_dimensions: Dimensions::new(2.0, 2.0),
                screen_dimensions: Dimensions::new(20.0, 20.0),
                lens_top_left_corner: math::Vector::new(1.0, 1.0, 0.0),
                u: math::Vector::new(-1.0, 0.0, 0.0),
                v: math::Vector::new(0.0, 1.0, 0.0),
                w: math::Vector::new(0.0, 0.0, -1.0)
            }
        );
    }
}
