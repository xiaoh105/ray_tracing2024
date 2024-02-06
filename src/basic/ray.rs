use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point,
    dir: Vec,
    tm: f64
}

impl Ray {
    pub fn origin(&self) -> &Point { &self.orig }
    pub fn direction(&self) -> &Vec { &self.dir }
    pub fn time(&self) -> f64 { self.tm }
    pub fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }
}

pub fn empty_ray() -> Ray {
    Ray {
        orig: empty_vec(),
        dir: empty_vec(),
        tm: 0.0
    }
}

pub fn ray(orig: Point, dir: Vec, tm: f64) -> Ray {
    Ray { orig, dir, tm }
}