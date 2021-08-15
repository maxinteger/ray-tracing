use crate::material::base::*;
use crate::ray::*;
use crate::vector::*;
use std::rc::Rc;
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn update(&mut self, other: &HitRecord) -> () {
        self.t = other.t;
        self.point = other.point;
        self.normal = other.normal;
        self.material = Some(Arc::clone(other.material.as_ref().unwrap()));
    }
}
