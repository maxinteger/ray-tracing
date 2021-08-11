use crate::hit_record::*;
use crate::objects::hittable::Hittable;
use crate::ray::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) -> () {
        self.objects.push(object)
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_hit_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.as_slice() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                closest_so_far = temp_hit_record.t;
                hit_record.update(&temp_hit_record);
            }
        }

        hit_anything
    }
}
