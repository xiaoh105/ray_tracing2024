mod basic;
mod hittable;
mod constants;
mod camera;
mod material;

use std::sync::Arc;

use basic::*;
use hittable::*;
use camera::*;
use crate::constants::{random_double, random_double_range};
use crate::material::*;

fn main() {
    let material_ground = Arc::new(lambertian::lambertian(color(0.5, 0.5, 0.5)));

    let mut world = empty_hittable_list();
    world.add(Arc::new(sphere(point(0.0, -1000.0, -1.0), 1000.0, material_ground)));
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = random_double();
            let center = point(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());
            if (center - point(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = rand_color() * rand_color();
                    let sphere_material = Arc::new(lambertian::lambertian(albedo));
                    world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = rand_color_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(metal::metal(albedo, fuzz));
                    world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(dielectics::dielectrics(1.5));
                    world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(dielectics::dielectrics(1.5));
    world.add(Arc::new(sphere(point(0.0, 1.0, 0.0), 1.0, material1)));
    let material2 = Arc::new(lambertian::lambertian(color(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere(point(-4.0, 1.0, 0.0), 1.0, material2)));
    let material3 = Arc::new(metal::metal(color(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere(point(4.0, 1.0, 0.0), 1.0, material3)));

    let cam = camera();

    render(Arc::new(cam), Arc::new(world));
}
