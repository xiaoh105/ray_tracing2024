use std::sync::Arc;
use crate::material::{lambertian, Scatter};
use super::{Hit, empty_record, HitRecord};
use super::super::basic::*;

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Arc<dyn Scatter + Sync + Send>,
    is_moving: bool,
    center_vec: Vec
}

pub fn empty_sphere() -> Sphere {
    Sphere {
        center: empty_point(),
        radius: 0.0,
        mat: Arc::new(lambertian::empty_lambertian()),
        is_moving: false,
        center_vec: empty_vec()
    }
}

pub fn sphere(center: Point, radius: f64, mat: Arc<dyn Scatter + Sync + Send>) -> Sphere {
    Sphere { center, radius, mat, is_moving: false, center_vec: empty_vec() }
}

pub fn moving_sphere(center1: Point, center2: Point, radius: f64, mat: Arc<dyn Scatter + Sync + Send>) -> Sphere {
    Sphere {
        center: center1,
        radius,
        mat,
        is_moving: true,
        center_vec: center2 - center1
    }
}

impl Sphere {
    pub fn center(&self, time: f64) -> Point {
        self.center + time * self.center_vec
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = if self.is_moving { self.center(r.time()) } else { self.center };
        let oc = *r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let mut rec = empty_record();
        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        Some(rec)
    }
}