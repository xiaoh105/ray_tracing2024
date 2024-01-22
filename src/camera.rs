use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::{io, thread};
use std::thread::{sleep};
use super::constants::*;
use super::hittable::{Hit, HitRes, HittableList};
use super::basic::*;

struct Position {
    i: i32,
    j: i32,
}

struct ThreadRes {
    x: i32,
    y: i32,
    color: Color,
}

fn thread_res(x: i32, y: i32, color: Color) -> ThreadRes { ThreadRes { x, y, color } }

fn position(i: i32, j: i32) -> Position { Position { i, j } }

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec,
    pixel_delta_v: Vec,
}

pub fn camera() -> Camera {
    let mut ret = Camera {
        aspect_ratio: ASPECT_RATIO,
        image_height: 0,
        image_width: IMAGE_WIDTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        center: center_point(),
        pixel00_loc: center_point(),
        pixel_delta_u: empty_vec(),
        pixel_delta_v: empty_vec(),
    };
    ret.initialize();
    ret
}

fn ray_color(cam: Arc<Camera>, r: &Ray, depth: i32, world: Arc<HittableList>) -> Color {
    if depth <= 0 {
        return black();
    }
    if let HitRes::Yes(hit_record) = world.hit(r, interval(0.001, INFINITY)) {
        let dir = hit_record.normal + rand_on_hemisphere(&hit_record.normal);
        0.5 * ray_color(cam, &ray(hit_record.p, dir), depth - 1, world)
    } else {
        let unit_direction = r.direction().unit();
        let a = (unit_direction.y() + 1.0) / 2.0;
        (1.0 - a) * white() + a * color(0.5, 0.7, 1.0)
    }
}

fn pixel_sample_square(cam: Arc<Camera>) -> Vec {
    let px = -0.5 + random_double();
    let py = -0.5 + random_double();
    px * cam.pixel_delta_u + py * cam.pixel_delta_v
}

fn get_ray(cam: Arc<Camera>, i: i32, j: i32) -> Ray {
    let pixel_center = cam.pixel00_loc + i as f64 * cam.pixel_delta_u + j as f64 * cam.pixel_delta_v;
    let pixel_sample = pixel_center + pixel_sample_square(cam.clone());
    ray(cam.center, pixel_sample - cam.center)
}

pub fn render(cam: Arc<Camera>, world: Arc<HittableList>) {
    let mut order: std::vec::Vec<Position> = std::vec::Vec::with_capacity((cam.image_width * cam.image_height) as usize);
    let mut result: std::vec::Vec<Color> = vec![black(); (cam.image_width * cam.image_height) as usize];
    for j in 0..cam.image_height {
        for i in 0..cam.image_width {
            order.push(position(i, j));
        }
    }
    random_shuffle(&mut order);

    let order = Arc::new(Mutex::new(order));
    let result = Arc::new(Mutex::new(result));
    let complete_num = Arc::new(Mutex::new(0));
    let start_time = Arc::new(Instant::now());

    println!("Start rendering.");
    let mut thread_handler = std::vec::Vec::new();

    {
        let image_height = cam.image_height;
        let image_width = cam.image_width;
        let complete_num = complete_num.clone();
        let start_time = start_time.clone();
        thread_handler.push(thread::spawn(move || {
            loop {
                sleep(Duration::from_millis(100));
                let complete_num = complete_num.lock().expect("Error occurred when trying to lock");
                let total_pixels = (image_width * image_height) as f64 / 1000.0;
                if start_time.elapsed().as_secs() == 0 {
                    print!("\rProgress:{}% ({:.1}k/{:.1}k pixels done).", ((*complete_num + 1) as f64 / total_pixels / 10.0) as i32,
                           (*complete_num + 1) as f64 / 1000.0, total_pixels);
                } else {
                    print!("\rProgress:{}% ({:.1}k/{:.1}k pixels done). Time left: {}s.",
                           ((*complete_num + 1) as f64 / total_pixels / 10.0) as i32, (*complete_num + 1) as f64 / 1000.0, total_pixels,
                           (start_time.elapsed().as_millis() as f64 / ((*complete_num + 1) as f64 / 1000.0) * (total_pixels - (*complete_num - 1) as f64 / 1000.0) / 1000.0) as i32);
                }
                if total_pixels == *complete_num as f64 / 1000.0 { return; }
            }
        }));
    }

    for _i in 0..THREADS_NUM {
        let world = world.clone();
        let samples_per_pixel = cam.samples_per_pixel;
        let max_depth = cam.max_depth;
        let order = order.clone();
        let result = result.clone();
        let image_width = cam.image_width;
        let cam = cam.clone();
        let complete_num = complete_num.clone();
        thread_handler.push(thread::spawn(move || {
            loop {
                let mut order = order.lock().expect("Error occurred when trying to lock.");
                if order.is_empty() { return; }
                let i = order[order.len() - 1].i;
                let j = order[order.len() - 1].j;
                order.pop();
                drop(order);
                let mut pixel_color = black();
                for _k in 0..samples_per_pixel {
                    let r = get_ray(cam.clone(), i, j);
                    pixel_color += ray_color(cam.clone(), &r, max_depth, world.clone());
                }
                let mut res = result.lock().expect("Error occurred when trying to lock.");
                res[(j * image_width + i) as usize] = pixel_color;
                drop(res);
                let mut complete_num = complete_num.lock().expect("Error occurred when trying to lock");
                *complete_num += 1;
                io::stdout().flush().expect("IO message error!");
            }
        }));
    }
    for i in thread_handler { i.join().expect("Error occurred when joining threads"); }
    println!("\nOutputting images.");
    let mut file = File::create("Image.ppm").unwrap();
    file.write(format!("P3\n{} {}\n255\n", cam.image_width, cam.image_height).as_ref()).unwrap();
    let result = result.lock().expect("Error occurred when trying to lock.");
    for i in 0..result.len() {
        result[i].write(&mut file, SAMPLES_PER_PIXEL);
    }
    println!("Output finished.");
    println!("Total time spent: {}ms", start_time.elapsed().as_millis());
}

impl Camera {
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
}