use std::fs::File;
use std::io;
use std::io::Write;
use std::time::Instant;
use super::constants::*;
use super::hittable::{Hit, HitRes, HittableList};
use super::basic::*;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec,
    pixel_delta_v: Vec,
}

pub fn camera() -> Camera {
    Camera {
        aspect_ratio: ASPECT_RATIO,
        image_height: 0,
        image_width: IMAGE_WIDTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        center: center_point(),
        pixel00_loc: center_point(),
        pixel_delta_u: empty_vec(),
        pixel_delta_v: empty_vec()
    }
}

impl Camera {
    fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
        if let HitRes::Yes(hit_record) = world.hit(r, interval(0.0, INFINITY)) {
            let dir = hit_record.normal;
            0.5 * color(dir.x() + 1.0, dir.y() + 1.0 , dir.z() + 1.0)
        } else {
            let unit_direction = r.direction();
            let a = (unit_direction.y() + 1.0) / 2.0;
            (1.0 - a) * white() + a * color(0.5, 0.7, 1.0)
        }
    }
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        if self.image_height < 1 { self.image_height = 1; }
        self.center = center_point();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        let viewport_u = vec(viewport_width, 0.0, 0.0);
        let viewport_v = vec(0.0, -viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left = self.center - vec(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
    fn pixel_sample_square(&self) -> Vec {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        ray(self.center, pixel_sample - self.center)
    }
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        let mut file = File::create("Image.ppm").unwrap();
        let start_time = Instant::now();
        file.write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_ref()).unwrap();
        println!("Start rendering.");
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = black();
                for _k in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, &world);
                }
                pixel_color.write(&mut file, self.samples_per_pixel);
            }
            if start_time.elapsed().as_secs() == 0 {
                print!("\rProgress:{}% ({}/{} rows done).", ((j + 1) as f64 / self.image_height as f64 * 100f64) as i32,
                       j + 1, self.image_height);
            } else {
                print!("\rProgress:{}% ({}/{} rows done). Time left: {}s.",
                       ((j + 1) as f64 / self.image_height as f64 * 100f64) as i32, j + 1, self.image_height,
                       (start_time.elapsed().as_millis() as f64 / (j + 1) as f64 * (self.image_height - j - 1) as f64 / 1000f64) as i32);
            }
            io::stdout().flush().expect("IO message error!");
        }
        println!("\nTotal time spent: {}ms", start_time.elapsed().as_millis())
    }
}