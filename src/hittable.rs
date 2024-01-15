mod sphere;
mod hittable_list;

pub use sphere::*;
pub use hittable_list::*;

use super::basic::*;
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub normal: Vec,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

fn empty_record() -> HitRecord {
    HitRecord { p: empty_point(), t: 0.0, normal: empty_vec(), front_face: false }
}

fn hit_record(p: Point, t: f64, normal: Vec, front_face: bool) -> HitRecord {
    HitRecord { p, t, normal, front_face }
}

pub enum HitRes {
    Yes(HitRecord),
    No
}

pub trait Hit {
    fn hit(&self, r: &Ray, ray_t: Interval) -> HitRes;
}