use std::sync::Arc;
use crate::basic::{Interval, interval, Ray};
use super::{Hit, HitRes};

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hit + Send + Sync>>
}

pub fn empty_hittable_list() -> HittableList {
    HittableList { objects: Vec::new() }
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }
    pub fn add<T: Hit + 'static + Send + Sync>(&mut self, object: Arc<T>) {
        self.objects.push(object);
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> HitRes {
        let mut hit_rec = HitRes::No;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let HitRes::Yes(tmp_rec) = object.hit(r, interval(ray_t.min, closest_so_far)) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                hit_rec = HitRes::Yes(tmp_rec);
            }
        }
        hit_rec
    }
}