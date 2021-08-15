use crate::hit_record::*;
use crate::ray::*;
use crate::vector::Color;
use rand::prelude::ThreadRng;

pub trait Material {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        ray_in: &Ray,
        hit_record: &HitRecord,
    ) -> (bool, Ray, Color);
}
