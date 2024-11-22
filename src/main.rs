//! Implementation of ray tracing in one weekend renderer.
//!
//!

use glam::Vec3;
use image::{ ImageBuffer, ImageEncoder, Rgb};
use rt_in_1_weekend::{
    materials::{Lambertian, Reflective, Refractive},
    parallel::{self, HEIGHT, WIDTH},
    scene::{Camera, Sphere},
    utils, BoxedSurface,
};
use std::path::Path;

fn world<'a>() -> Vec<BoxedSurface> {
    let mut world: Vec<BoxedSurface> = vec![];

    let diffuse1 = Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let diffuse2 = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let reflective = Box::new(Reflective::new(Vec3::new(0.8, 0.6, 0.2)));
    let refractive = Box::new(Refractive::new(1.5));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        diffuse1,
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        diffuse2,
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-1.1, -0.1, -1.0),
        0.4,
        // reflective.clone(),
        refractive,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.1, 0.0, -1.0),
        0.4,
        reflective,
    )));

    world
}

fn main() {
    let t = utils::Timer::new();
    let camera = Camera::default();
    let vec = parallel::render(camera, world());
    let mut buf = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for ((.., pixel), color) in buf.enumerate_pixels_mut().zip(vec) {
        *pixel = color
    }
    utils::save_image(buf, &Path::new("output/scene.png"));
    t.end();
}
