mod sphere;
mod hittable_list;

use std::rc::Rc;
pub use sphere::*;
pub use hittable_list::*;
use crate::material::{lambertian, Scatter};

use super::basic::*;
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub normal: Vec,
    pub front_face: bool,
    pub mat: Rc<dyn Scatter>
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

fn empty_record() -> HitRecord {
    HitRecord { p: empty_point(), t: 0.0, normal: empty_vec(), front_face: false, mat: Rc::new(lambertian::empty_lambertian()) }
}

fn hit_record(p: Point, t: f64, normal: Vec, front_face: bool, mat: Rc<dyn Scatter>) -> HitRecord {
    HitRecord { p, t, normal, front_face, mat }
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}