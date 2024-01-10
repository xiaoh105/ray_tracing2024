mod basic;

use std::fs::File;
use std::io;
use std::io::Write;
use std::time::Instant;
use basic::*;

enum HitResult {
    Yes(f64), No
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> HitResult {
    let oc = &(*(*r).origin() - *center);
    let a = r.direction().length_squared();
    let half_b = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant >= 0f64 {
        HitResult::Yes((-half_b - discriminant.sqrt()) / a)
    } else { HitResult::No }
}

fn ray_color(r: &Ray) -> Color {
    if let HitResult::Yes(t) = hit_sphere(&point(0f64, 0f64, -1f64), 0.5, r) {
        let dir = (r.at(t) - point(0.0, 0.0, -1.0)).unit();
        0.5 * color(dir.x() + 1.0, dir.y() + 1.0 , dir.z() + 1.0)
    } else {
        let unit_direction = r.direction();
        let a = (unit_direction.y() + 1.0) / 2.0;
        (1.0 - a) * white() + a * color(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut file = File::create("Image.ppm").unwrap();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0;

    let camera_center = center_point();
    let viewport_u = vec(viewport_width, 0f64, 0f64);
    let viewport_v = vec(0f64, -viewport_height, 0f64);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - vec(0f64, 0f64, focal_length)
        - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;


    let start_time = Instant::now();
    file.write(format!("P3\n{image_width} {image_height}\n255\n").as_ref()).unwrap();
    println!("Start rendering.");
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = ray(pixel_center, ray_direction);

            let pixel_color = ray_color(&r);
            pixel_color.write(&mut file);
        }
        if start_time.elapsed().as_secs() == 0 {
            print!("\rProgress:{}% ({}/{} rows done).", ((j + 1) as f64 / image_height as f64 * 100f64) as i32,
                   j + 1, image_height);
        } else {
            print!("\rProgress:{}% ({}/{} rows done). Time left: {}s.",
                   ((j + 1) as f64 / image_height as f64 * 100f64) as i32, j + 1, image_height,
                   (start_time.elapsed().as_millis() as f64 / (j + 1) as f64 * (image_height - j - 1) as f64 / 1000f64) as i32);
        }
        io::stdout().flush().expect("IO message error!");
    }
    println!("\nTotal time spent: {}ms", start_time.elapsed().as_millis())
}
