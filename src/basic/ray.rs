use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point,
    dir: Vec
}

impl Ray {
    pub fn origin(&self) -> &Point { &self.orig }
    pub fn direction(&self) -> &Vec { &self.dir }
    pub fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }
}

pub fn empty_ray() -> Ray {
    Ray {
        orig: empty_vec(),
        dir: empty_vec()
    }
}

pub fn ray(orig: Point, dir: Vec) -> Ray {
    Ray { orig, dir }
}