use super::{light, math, util};

pub trait Object {
    fn is_intersected_by(&self, ray: &util::Ray) -> bool;
    fn get_point_intersected_by(&self, ray: &util::Ray) -> math::Vector;
    fn compute_color(
        &self,
        point: &math::Vector,
        ray: &util::Ray,
        light: &light::Light,
    ) -> util::Color;
}

#[derive(Copy, Clone)]
pub struct Surface {
    diffuse_constant: math::Vector,
    specular_constant: math::Vector,
}

impl Surface {
    pub fn new(dc: math::Vector, sc: math::Vector) -> Surface {
        Surface {
            diffuse_constant: dc,
            specular_constant: sc,
        }
    }
}

pub struct Sphere {
    pub center: math::Vector,
    pub radius: f32,
    pub surface: Surface,
}

impl Sphere {
    pub fn new(center: math::Vector, radius: f32, surface: Surface) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            surface: surface,
        }
    }

    fn compute_normal(&self, point: &math::Vector) -> math::Vector {
        let normal = (*point - self.center) / self.radius;
        normal
    }
}

impl Object for Sphere {
    fn is_intersected_by(&self, ray: &util::Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = math::Vector::dot_product(ray.direction, ray.direction);
        let b = math::Vector::dot_product(oc, ray.direction) * 2.0;
        let c = math::Vector::dot_product(oc, oc) - self.radius.powi(2);
        let discriminant = b * b - a * c * 4.0;
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        let mut is_intersected = false;
        if t > 0.0 && t2 > 0.0 {
            is_intersected = true;
        }
        return is_intersected;
    }

    fn get_point_intersected_by(&self, ray: &util::Ray) -> math::Vector {
        let oc = ray.origin - self.center;
        let a = math::Vector::dot_product(ray.direction, ray.direction);
        let b = math::Vector::dot_product(oc, ray.direction) * 2.0;
        let c = math::Vector::dot_product(oc, oc) - self.radius.powi(2);
        let discriminant = b * b - a * c * 4.0;

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let point = ray.origin + ray.direction * t;
        return point;
    }

    fn compute_color(
        &self,
        point: &math::Vector,
        ray: &util::Ray,
        light: &light::Light,
    ) -> util::Color {
        let normal = self.compute_normal(point);

        let v = -ray.direction;
        let half_vector = ((light.position - *point).normalize() + v).normalize();
        let specular = self.surface.specular_constant
            * light.intensity
            * math::Vector::dot_product(normal, half_vector)
                .powf(50.0)
                .max(0.0);

        let diffuse = self.surface.diffuse_constant
            * light.intensity
            * math::Vector::dot_product(normal, (light.position - *point).normalize()).max(0.0);
        let illumination = diffuse + specular;
        let mut color = util::Color::new(0.0, 0.0, 0.0);

        if illumination.x > 1.0 {
            color.r = 1.0;
        } else {
            color.r = illumination.x;
        }

        if illumination.y > 1.0 {
            color.g = 1.0;
        } else {
            color.g = illumination.y;
        }

        if illumination.z > 1.0 {
            color.b = 1.0;
        } else {
            color.b = illumination.z;
        }

        return color;
    }
}
