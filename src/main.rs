extern crate rand;

use std::time::SystemTime;

mod camera;
mod image;
mod light;
mod math;
mod object;
mod rendering;
mod scene;
mod util;

fn main() {
    // Creating camera
    let position = math::Vector::new(0.0, 0.0, -0.4);
    let direction = math::Vector::new(0.0, 0.0, 1.0);
    let camera = camera::Camera::new(
        position,
        direction,
        1.0,
        camera::Dimensions::new(0.45, 0.8),
        camera::Dimensions::new(720 as f32, 1280 as f32),
    );

    // Creating surfaces for the objects
    let red_surface = object::Surface::new(
        math::Vector::new(1.0, 0.0, 0.0),
        math::Vector::new(0.7, 0.7, 0.7),
    );
    let green_surface = object::Surface::new(
        math::Vector::new(0.0, 1.0, 0.0),
        math::Vector::new(0.7, 0.7, 0.7),
    );
    let orange_surface = object::Surface::new(
        math::Vector::new(1.0, 0.65, 0.0),
        math::Vector::new(0.7, 0.7, 0.7),
    );
    let blue_surface = object::Surface::new(
        math::Vector::new(0.0, 0.0, 1.0),
        math::Vector::new(0.4, 0.4, 0.4),
    );

    // Creating objects
    let red_sphere = object::Sphere::new(math::Vector::new(0.3, 0.0, 1.0), 0.1, red_surface);
    let green_sphere = object::Sphere::new(math::Vector::new(-0.3, 0.0, 1.0), 0.1, green_surface);
    let orange_sphere = object::Sphere::new(math::Vector::new(0.0, 0.0, 1.5), 0.4, orange_surface);
    let blue_plane = object::Plane::new(
        math::Vector::new(0.0, 1.0, 0.0),
        math::Vector::new(0.0, -0.4, 0.0),
        blue_surface,
    );
    let objects: Vec<Box<object::Object>> = vec![
        Box::new(green_sphere),
        Box::new(red_sphere),
        Box::new(orange_sphere),
        Box::new(blue_plane),
    ];

    // Creating lights
    let right_light = light::Light::new(math::Vector::new(-0.8, 0.5, 0.0), 0.8);
    let left_light = light::Light::new(math::Vector::new(0.8, 0.5, 0.0), 0.8);
    let lights: Vec<light::Light> = vec![left_light];

    // Creating scene
    let scene = scene::Scene::new(lights, objects);

    // Creating renderer
    let renderer = rendering::Renderer::new(camera, scene);

    let time_now = SystemTime::now();
    let rendering_result = renderer.render();
    match time_now.elapsed() {
        Ok(elapsed) => {
            println!(
                "Elapsed time during rendering: {} seconds",
                elapsed.as_secs()
            );
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }

    match rendering_result.write("spheres.ppm") {
        Ok(result) => result,
        Err(error) => {
            panic!(
                "Could not write the rendering result, gerenerating this error: {}",
                error
            );
        }
    }
}
