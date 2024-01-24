pub mod lambertian;
pub mod metal;
pub mod dielectics;

use crate::basic::{black, Color, empty_ray, Ray};
use crate::hittable::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray
}

pub fn empty_scatter_record() -> ScatterRecord {
    ScatterRecord{
        attenuation: black(),
        scattered: empty_ray()
    }
}

pub fn scatter_record(attenuation: Color, scattered: Ray) -> ScatterRecord { ScatterRecord{ attenuation, scattered } }

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}