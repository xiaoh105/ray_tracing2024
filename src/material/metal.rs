use crate::basic::{black, Color, dot, rand_unit_vec, Ray, ray, reflect};
use crate::hittable::HitRecord;
use crate::material::{Scatter, scatter_record, ScatterRecord};

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

pub fn empty_metal() -> Metal { Metal{ albedo: black(), fuzz: 0.0 } }
pub fn metal(albedo: Color, fuzz: f64) -> Metal { Metal{ albedo, fuzz: fuzz.abs().min(1.0) } }

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&r_in.direction().unit(), &rec.normal);
        let ret = scatter_record(self.albedo, ray(rec.p, reflected + self.fuzz * rand_unit_vec(), r_in.time()));
        if dot(ret.scattered.direction(), &rec.normal) > 0.0 {
            Some(ret)
        } else { None }
    }
}