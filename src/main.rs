mod basic;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use basic::*;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction();
    let a = (unit_direction.y() + 1.0) / 2.0;
    (1.0 - a) * white() + a * color(0.5, 0.7, 1.0)
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
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let r = ray(pixel_center, ray_direction);

            let pixel_color = ray_color(&r);
            pixel_color.write(&mut file);
        }
        println!("{}/{} rows done.", j + 1, image_height);
    }
    println!("Total time spent: {}ms", start_time.elapsed().as_millis())
}
