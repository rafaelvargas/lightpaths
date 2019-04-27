use super::{light, math, object, util};

const BACKGROUND_COLOR: util::Color = util::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

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
        let mut shortest_distance = std::f32::MAX;
        let mut closest_point = math::Vector::new(0.0, 0.0, 0.0);
        let mut closest_object_index = 0;
        let mut has_intersection = false;

        for (i, object) in self.objects.iter().enumerate() {
            if object.is_intersected_by(ray) {
                has_intersection = true;
                let point = object.get_point_intersected_by(ray);
                let distance_to_point = (point - ray.origin).magnitude();
                if distance_to_point < shortest_distance {
                    shortest_distance = distance_to_point;
                    closest_point = point;
                    closest_object_index = i;
                }
            }
        }

        if has_intersection {
            for light in self.lights.iter() {
                let shadow_ray_direction = (light.position - closest_point).normalize();
                let shadow_ray = util::Ray::new(
                    closest_point + (shadow_ray_direction * std::f32::EPSILON),
                    shadow_ray_direction,
                );
                let mut is_shadow_ray_intersecting_object = false;
                for object in self.objects.iter() {
                    if object.is_intersected_by(&shadow_ray) {
                        is_shadow_ray_intersecting_object = true;
                    }
                }
                if !is_shadow_ray_intersecting_object {
                    color += self.objects[closest_object_index].compute_color(
                        &closest_point,
                        &ray,
                        &light,
                    );
                }
            }
        } else {
            color = BACKGROUND_COLOR;
        }
        color.clamp();

        return color;
    }
}
