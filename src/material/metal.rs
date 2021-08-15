use crate::hit_record::HitRecord;
use crate::material::base::Material;
use crate::ray::Ray;
use crate::vector::{random_unit_vector, reflect, Color};
use rand::prelude::ThreadRng;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> (bool, Ray, Color) {
        let reflected = reflect(ray_in.direction.unit_vector(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * random_unit_vector(rng),
        );
        let is_scatter = scattered.direction.dot(hit_record.normal) > 0.0;
        return (is_scatter, scattered, self.albedo);
    }
}
