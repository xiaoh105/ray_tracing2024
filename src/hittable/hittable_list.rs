use std::rc::Rc;
use crate::basic::Ray;
use super::{empty_record, Hit, HitRes};

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hit>>
}

pub fn empty_hittable_list() -> HittableList {
    HittableList { objects: Vec::new() }
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }
    pub fn add<T: Hit+'static>(&mut self, object: Rc<T>) {
        self.objects.push(object);
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitRes {
        let mut hit_rec = HitRes::No;
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let HitRes::Yes(tmp_rec) = object.hit(r, ray_tmin, closest_so_far) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                hit_rec = HitRes::Yes(tmp_rec);
            }
        }
        hit_rec
    }
}