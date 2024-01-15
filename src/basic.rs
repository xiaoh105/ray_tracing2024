pub mod vec;
pub mod color;
pub mod ray;
pub mod interval;

use super::constants::*;

pub type Point = Vec;

pub use vec::*;
pub use color::*;
pub use ray::*;
pub use interval::*;