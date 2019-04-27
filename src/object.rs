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
        let color = util::Color::new(illumination.x, illumination.y, illumination.z);
        return color;
    }
}

pub struct Plane {
    normal: math::Vector,
    distance_to_origin: f32,
    surface: Surface,
}

impl Plane {
    pub fn new(normal: math::Vector, point: math::Vector, surface: Surface) -> Plane {
        Plane {
            normal: normal,
            distance_to_origin: -math::Vector::dot_product(point, normal),
            surface: surface,
        }
    }
}

impl Object for Plane {
    fn is_intersected_by(&self, ray: &util::Ray) -> bool {
        let vd = math::Vector::dot_product(self.normal, ray.direction);
        let vo = math::Vector::dot_product(self.normal, ray.origin);
        let t = -(vo + self.distance_to_origin) / vd;
        return t > 0.0;
    }

    fn get_point_intersected_by(&self, ray: &util::Ray) -> math::Vector {
        let vd = math::Vector::dot_product(self.normal, ray.direction);
        let vo = math::Vector::dot_product(self.normal, ray.origin);
        let t = -(vo + self.distance_to_origin) / vd;
        let point = ray.origin + ray.direction * t;
        return point;
    }

    fn compute_color(
        &self,
        point: &math::Vector,
        ray: &util::Ray,
        light: &light::Light,
    ) -> util::Color {
        let v = -ray.direction;
        let half_vector = ((light.position - *point).normalize() + v).normalize();
        let specular = self.surface.specular_constant
            * light.intensity
            * math::Vector::dot_product(self.normal, half_vector)
                .powf(50.0)
                .max(0.0);

        let diffuse = self.surface.diffuse_constant
            * light.intensity
            * math::Vector::dot_product(self.normal, (light.position - *point).normalize())
                .max(0.0);
        let illumination = diffuse + specular;
        let color = util::Color::new(illumination.x, illumination.y, illumination.z);

        return color;
    }
}
