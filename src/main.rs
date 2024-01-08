mod basic;

use std::fs::File;
use std::io::Write;
use basic::{vec::*, color::*};

fn main() {
    let mut file = File::create("Image.ppm").unwrap();
    let image_width = 256;
    let image_height = 256;
    file.write(format!("P3\n{image_width} {image_height}\n255\n").as_ref()).unwrap();
    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_color = color((j as f64) / (image_width - 1) as f64, (i as f64) / (image_height - 1) as f64, 0f64);
            pixel_color.write(&mut file);
        }
        println!("{}/{} rows done.", i + 1, image_height);
    }
}
