use crate::hit_record::HitRecord;
use crate::material::base::Material;
use crate::ray::Ray;
use crate::vector::{random_unit_vector, Color};
use rand::prelude::ThreadRng;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> (bool, Ray, Color) {
        let scatter_direction = hit_record.normal + random_unit_vector(rng);

        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new(hit_record.point, scatter_direction);
        return (true, scattered, self.albedo);
    }
}
