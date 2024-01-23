use std::fs::File;
use std::io::Write;
use std::ops::*;
use crate::basic::{Interval, interval};
use crate::constants::linear_to_gamma;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn r(&self) -> f64 { self.r }
    pub fn g(&self) -> f64 { self.g }
    pub fn b(&self) -> f64 { self.b }
    pub fn write(&self, output: &mut File, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;

        let r = self.r * scale;
        let g = self.g * scale;
        let b = self.b * scale;

        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        let intensity: Interval = interval(0.000, 0.999);

        let r = (256.0 * intensity.clamp(r)) as i32;
        let g = (256.0 * intensity.clamp(g)) as i32;
        let b = (256.0 * intensity.clamp(b)) as i32;

        output.write(format!("{r} {g} {b}\n").as_ref())
            .expect("Error occurred when writing image to file.");
    }
}

impl Add for Color {
    type Output =Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output =Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Index<i32> for Color {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        if index == 0 {
            &self.r
        } else if index == 1 {
            &self.g
        } else if index == 2 {
            &self.b
        } else {
            panic!("Color index={} out of range!", index)
        }
    }
}

impl IndexMut<i32> for Color {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        if index == 0 {
            &mut self.r
        } else if index == 1 {
            &mut self.g
        } else if index == 2 {
            &mut self.b
        } else {
            panic!("Color mut index={} out of range!", index)
        }
    }
}

pub fn white() -> Color {
    Color { r: 1f64, g: 1f64, b: 1f64 }
}

pub fn black() -> Color {
    Color { r: 0f64, g: 0f64, b: 0f64 }
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

pub fn write_color(output: &mut File, color: &Color) {
    let r = (color.r * 255.999) as i32;
    let g = (color.g * 255.999) as i32;
    let b = (color.b * 255.999) as i32;
    output.write(format!("{r} {g} {b}\n").as_ref())
        .expect("Error occurred when writing image to file.");
}