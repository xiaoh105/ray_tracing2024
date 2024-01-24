use crate::basic::{dot, empty_vec, Ray, ray, reflect, refract, white};
use crate::constants::random_double;
use crate::hittable::HitRecord;
use crate::material::{Scatter, scatter_record, ScatterRecord};

pub struct Dielectrics {
    ir: f64 // Index of refraction
}

pub fn empty_dielectrics() -> Dielectrics { Dielectrics{ ir: 1.0 } }
pub fn dielectrics(ir: f64) -> Dielectrics { Dielectrics{ ir } }

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Scatter for Dielectrics {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = r_in.direction().unit();
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction =
        if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > random_double() { // Reflect
            reflect(&unit_direction, &rec.normal)
        } else { // Refract
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        Some(scatter_record(white(), ray(rec.p, direction)))
    }
}