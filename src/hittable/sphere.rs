use std::sync::Arc;
use crate::material::{lambertian, Scatter};
use super::{Hit, empty_record, HitRecord};
use super::super::basic::*;

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Arc<dyn Scatter + Sync + Send>
}

pub fn empty_sphere() -> Sphere {
    Sphere {
        center: empty_point(),
        radius: 0.0,
        mat: Arc::new(lambertian::empty_lambertian())
    }
}

pub fn sphere(center: Point, radius: f64, mat: Arc<dyn Scatter + Sync + Send>) -> Sphere {
    Sphere { center, radius, mat }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let mut rec = empty_record();
        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        Some(rec)
    }
}