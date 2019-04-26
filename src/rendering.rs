use super::{camera, image, scene, util};
use rand::prelude::*;

const ITERATIONS_PER_PIXEL: u32 = 100;

pub struct Renderer {
    camera: camera::Camera,
    scene: scene::Scene,
}

impl Renderer {
    pub fn new(c: camera::Camera, s: scene::Scene) -> Renderer {
        Renderer {
            camera: c,
            scene: s,
        }
    }

    pub fn render(&self) -> image::Image {
        let mut rng = rand::thread_rng();
        let mut image = image::Image::new(720, 1280);

        for i in 0..image.get_height() {
            for j in 0..image.get_width() {
                let mut sum_pixel_color = util::Color::new(0.0, 0.0, 0.0);
                for _k in 0..ITERATIONS_PER_PIXEL {
                    let ray = self.camera.generate_ray(
                        i as f32 + rng.gen_range(-0.5, 0.5),
                        j as f32 + rng.gen_range(-0.5, 0.5),
                    );
                    sum_pixel_color += self.scene.compute_color(&ray);
                }
                let average_pixel_color = sum_pixel_color / ITERATIONS_PER_PIXEL;
                image[i][j] = vec![
                    (average_pixel_color.r * 255.0) as u8,
                    (average_pixel_color.g * 255.0) as u8,
                    (average_pixel_color.b * 255.0) as u8,
                ];
            }
        }
        return image;
    }
}
