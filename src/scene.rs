use super::{light, object, util};

pub struct Scene {
    lights: Vec<light::Light>,
    objects: Vec<Box<object::Object>>,
}

impl Scene {
    pub fn new(lights: Vec<light::Light>, objects: Vec<Box<object::Object>>) -> Scene {
        Scene {
            lights: lights,
            objects: objects,
        }
    }

    pub fn compute_color(&self, ray: &util::Ray) -> util::Color {
        let mut color = util::Color::new(0.0, 0.0, 0.0);
        for object in self.objects.iter() {
            if object.is_intersected_by(ray) {
                let point = object.get_point_intersected_by(ray);
                for light in self.lights.iter() {
                    color += object.compute_color(&point, &ray, &light);
                }
            } else {
                color += util::Color::new(0.0, 0.0, 0.0);
            }
        }
        color.clamp();
        return color;
    }
}
