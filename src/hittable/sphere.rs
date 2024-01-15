use super::{HitRes, Hit, empty_record};
use super::super::basic::*;

pub struct Sphere {
    center: Point,
    radius: f64
}

pub fn empty_sphere() -> Sphere {
    Sphere {
        center: empty_point(),
        radius: 0.0
    }
}

pub fn sphere(center: Point, radius: f64) -> Sphere {
    Sphere { center, radius }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitRes {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return HitRes::No;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            let root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return HitRes::No;
            }
        }
        let mut rec = empty_record();
        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        HitRes::Yes(rec)
    }
}