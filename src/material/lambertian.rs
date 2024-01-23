use crate::basic::{black, Color, rand_unit_vec, Ray, ray};
use crate::hittable::HitRecord;
use crate::material::{Scatter, scatter_record, ScatterRecord};

pub struct Lambertian {
    albedo: Color
}

pub fn empty_lambertian() -> Lambertian { Lambertian{ albedo: black() } }

pub fn lambertian(albedo: Color) -> Lambertian { Lambertian{ albedo } }

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + rand_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some(scatter_record(self.albedo, ray(rec.p, scatter_direction)))
    }
}