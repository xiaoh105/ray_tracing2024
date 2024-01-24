use crate::constants::INFINITY;

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

pub const EMPTY: Interval = Interval { min: INFINITY, max: -INFINITY };
pub const UNIVERSE: Interval = Interval { min: -INFINITY, max: INFINITY };

pub fn empty_interval() -> Interval {
    Interval { min: INFINITY, max: -INFINITY }
}

pub fn interval(min: f64, max: f64) -> Interval {
    Interval { min, max }
}

impl Interval {
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min } else if x > self.max { self.max } else { x }
    }
}