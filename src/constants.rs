use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;
use crate::basic::{Point, point, vec, Vec};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn linear_to_gamma(val: f64) -> f64 { val.sqrt() }

/*
    From below are random functions.
*/

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + random_double() * (max - min)
}

pub fn random_shuffle<T>(sequence: &mut std::vec::Vec<T>) { sequence.shuffle(&mut thread_rng()); }

/*
    From below are camera parameters.
*/

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: i32 = 1200;
pub const SAMPLES_PER_PIXEL: i32 = 500;
pub const MAX_DEPTH: i32 = 50;
pub const VFOV: f64 = 20.0;
pub const LOOK_FROM: Point = point(13.0, 2.0, 3.0);
pub const LOOK_AT: Point = point(0.0, 0.0, 0.0);
pub const VUP: Vec = vec(0.0, 1.0, 0.0);

/*
    From below are multithreading parameters.
 */
pub const THREADS_NUM: i32 = 7;