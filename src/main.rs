mod basic;
mod hittable;
mod constants;
mod camera;

use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;
use std::time::Instant;

use basic::*;
use hittable::*;
use constants::*;
use camera::*;

fn main() {
    let mut world = empty_hittable_list();
    world.add(Rc::new(sphere(point(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere(point(0.0, -100.5, -1.0), 100.0)));

    let mut cam = camera();

    cam.render(&world);
}
