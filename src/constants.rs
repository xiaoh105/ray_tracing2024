use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 { degrees * PI / 180.0 }

pub fn linear_to_gamma(val: f64) -> f64 { val.sqrt() }

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + random_double() * (max - min)
}

pub fn random_shuffle<T>(sequence: &mut std::vec::Vec<T>) { sequence.shuffle(&mut thread_rng()); }

/*
    From below are camera constants.
*/

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 400;
pub const SAMPLES_PER_PIXEL: i32 = 100;
pub const MAX_DEPTH: i32 = 50;

/*
    From below are sync constants.
 */
pub const THREADS_NUM: i32 = 10;