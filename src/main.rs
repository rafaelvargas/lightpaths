extern crate rand;

use rand::prelude::*;

mod image;
mod math;
mod objects;
mod utils;

fn is_intercepting(ray: utils::Ray, sphere: &objects::Sphere) -> bool {
    let oc = ray.origin - sphere.center;
    let a = math::Vector::dot_product(ray.direction, ray.direction);
    let b = math::Vector::dot_product(oc, ray.direction) * 2.0;
    let c = math::Vector::dot_product(oc, oc) - sphere.radius.powi(2);

    let discriminant = b * b - a * c * 4.0;
    return discriminant > 0.0;
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut image = image::Image::new(100, 100);

    let position = math::Vector::new(0.0, 0.0, 0.0);
    let direction = math::Vector::new(0.0, 0.0, 1.0);
    let camera = utils::Camera::new(
        position,
        direction,
        1.0,
        utils::Dimensions::new(2.0, 2.0),
        utils::Dimensions::new(image.get_height() as f32, image.get_width() as f32),
    );
    
    let sphere = objects::Sphere::new(math::Vector::new(0.0, 0.0, 1.0), 0.5);


    let mut red: u64;
    let mut green: u64;
    let mut blue: u64;
    for i in 0..image.get_height() {
        for j in 0..image.get_width() {
            red = 0;
            green = 0;
            blue = 0;
            for k in 0..100 {
                if is_intercepting(
                    camera.generate_ray(
                        i as f32 + rng.gen_range(-0.5, 0.5),
                        j as f32 + rng.gen_range(-0.5, 0.5),
                    ),
                    &sphere,
                ) {
                    red += 255;
                } else {
                    blue += 255;
                }
            }
            image[i][j][0] = (red / 100) as u8;
            image[i][j][1] = (green / 100) as u8;
            image[i][j][2] = (blue / 100) as u8;
        }
    }

    match image.write("sphere.ppm") {
        Ok(result) => result,
        Err(error) => {
            panic!(
                "Could not write the image, gerenerating this error: {}",
                error
            );
        }
    }
}
