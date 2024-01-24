mod basic;
mod hittable;
mod constants;
mod camera;
mod material;

use std::sync::Arc;

use basic::*;
use hittable::*;
use camera::*;
use crate::material::*;

fn main() {
    let material_ground = Arc::new(lambertian::lambertian(color(0.8, 0.8, 0.0)));
    let material_center = Arc::new(lambertian::lambertian(color(0.1, 0.2, 0.5)));
    let material_left = Arc::new(dielectics::dielectrics(1.5));
    let material_right = Arc::new(metal::metal(color(0.8, 0.6, 0.2), 0.0));

    let mut world = empty_hittable_list();
    world.add(Arc::new(sphere(point(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(sphere(point(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Arc::new(sphere(point(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Arc::new(sphere(point(-1.0, 0.0, -1.0), -0.4, material_left)));
    world.add(Arc::new(sphere(point(1.0, 0.0, -1.0), 0.5, material_right)));

    let cam = camera();

    render(Arc::new(cam), Arc::new(world));
}
