use std::ops::*;
use crate::basic::Point;
use crate::constants::{random_double, random_double_range};

#[derive(Debug, Copy, Clone)]
pub struct Vec {
    x: f64,
    y: f64,
    z: f64,
}


impl Vec {
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit(&self) -> Self {
        *self / self.length()
    }
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
}

impl Neg for Vec {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add for Vec {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f64> for Vec {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec> for f64 {
    type Output = Vec;
    fn mul(self, rhs: Vec) -> Self::Output {
        Vec {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl DivAssign<f64> for Vec {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<i32> for Vec {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        if index == 0 {
            &self.x
        } else if index == 1 {
            &self.y
        } else if index == 2 {
            &self.z
        } else {
            panic!("Vec index={} out of range!", index)
        }
    }
}

impl IndexMut<i32> for Vec {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else if index == 2 {
            &mut self.z
        } else {
            panic!("Vec mut index={} out of range!", index)
        }
    }
}

pub fn empty_vec() -> Vec {
    Vec { x: 0f64, y: 0f64, z: 0f64 }
}

pub const fn vec(x: f64, y: f64, z: f64) -> Vec {
    Vec { x, y, z }
}

pub fn rand_vec() -> Vec { vec(random_double(), random_double(), random_double()) }

pub fn rand_vec_range(min: f64, max: f64) -> Vec {
    vec(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
}

pub fn center_point() -> Point {
    Point { x:0f64, y: 0f64, z: 0f64 }
}

pub fn empty_point() -> Point { Point { x: 0.0, y: 0.0, z: 0.0 } }

pub const fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
}

pub fn dot(lhs: &Vec, rhs: &Vec) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn cross(lhs: &Vec, rhs: &Vec) -> Vec {
    Vec {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x
    }
}

pub fn reflect(v: &Vec, n: &Vec) -> Vec { *v - 2.0 * dot(v, n) * (*n) }

/// Refract a ray with direction v and outward surface normal n, etai_over_etat is the two material's ratio of the index of refraction.
/// Notice: v must be a unit vector.
pub fn refract(v: &Vec, n: &Vec, etai_over_etat: f64) -> Vec {
    let cos_theta = dot(&-(*v), n).min(1.0);
    let r_out_perp = etai_over_etat * (*v + cos_theta * (*n));
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*n);
    r_out_parallel + r_out_perp
}

pub fn unit(v: &Vec) -> Vec {
    *v / v.length()
}

fn rand_in_unit_sphere() -> Vec {
    loop {
        let p = rand_vec_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

pub fn rand_unit_vec() -> Vec { rand_in_unit_sphere().unit() }

pub fn rand_on_hemisphere(normal: &Vec) -> Vec {
    let on_unit_sphere = rand_unit_vec();
    if dot(normal, &on_unit_sphere) > 0.0 { on_unit_sphere } else { -on_unit_sphere }
}

pub fn rand_in_unit_disk() -> Vec {
    loop {
        let p = vec(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 { return p; }
    }
}