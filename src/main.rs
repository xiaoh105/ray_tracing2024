mod basic;
mod hittable;
mod constants;
mod camera;

use std::sync::Arc;

use basic::*;
use hittable::*;
use camera::*;

fn main() {
    let mut world = empty_hittable_list();
    world.add(Arc::new(sphere(point(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(sphere(point(0.0, -100.5, -1.0), 100.0)));

    let cam = camera();

    render(Arc::new(cam), Arc::new(world));
}
