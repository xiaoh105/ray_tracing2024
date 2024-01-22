mod basic;
mod hittable;
mod constants;
mod camera;

use std::rc::Rc;

use basic::*;
use hittable::*;
use camera::*;

fn main() {
    let mut world = empty_hittable_list();
    world.add(Rc::new(sphere(point(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere(point(0.0, -100.5, -1.0), 100.0)));

    let mut cam = camera();

    cam.render(&world);
}
