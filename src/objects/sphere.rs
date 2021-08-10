use crate::hit_record::*;
use crate::objects::hittable::Hittable;
use crate::ray::*;
use crate::vector::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, mut hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            false
        } else {
            let sqrt_discriminant = discriminant.sqrt();
            let root = (-half_b - sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                let root = (-half_b + sqrt_discriminant) / a;
                if root < t_min || t_max < root {
                    return false;
                }
            }

            hit_record.t = root;
            hit_record.point = ray.at(hit_record.t);
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.set_face_normal(ray, outward_normal);
            true
        }
    }
}
