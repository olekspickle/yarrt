//! Implementation of ray tracing in one weekend renderer.
//!
//!

use glam::Vec3;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rt_in_1_weekend::{
    color,
    materials::{Lambertian, Metal},
    scene::{Camera, Sphere},
    utils, Surface,
};
use std::path::Path;

const DIM: u32 = 200;
const DEPTH: i32 = 10;

fn world<'a>() -> Vec<Box<dyn Surface>> {
    let mut world: Vec<Box<dyn Surface>> = vec![];

    let diffuse1 = Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    let diffuse2 = Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let reflective1 = Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));
    let reflective2 = Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.1, -1.0),
        0.5,
        diffuse1,
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        diffuse2,
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.2, -1.0),
        0.3,
        reflective1,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.2, -1.0),
        0.3,
        reflective2,
    )));

    world
}

fn main() {
    let t = utils::Timer::new();
    // ns is just step count for antialiasing
    let (w, h, ns) = (DIM as f32 * 2.0, DIM as f32, 100);
    let mut buf = ImageBuffer::new(w as u32, h as u32);

    let world: Vec<Box<dyn Surface>> = world();
    let camera = Camera::default();

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let mut col = Vec3::ZERO;
        let (x, y) = (x as f32, y as f32);

        for _ in 0..ns {
            let mut rng = rand::thread_rng();
            let rand = rng.gen_range(0.0..1.0);

            // This two are pixel's relative coordinates on the screen
            let (u, v) = (((x + rand) / w), ((y + rand) / h));
            let ray = camera.get_ray(u, v);
            // let p = ray.point_at(2.0);
            col += color(&ray, &world, DEPTH);
        }

        col /= ns as f32;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        *pixel = Rgb([
            (col[0] * 255.99) as u8,
            (col[1] * 255.99) as u8,
            (col[2] * 255.99) as u8,
        ]);
    }

    utils::save_image(buf, &Path::new("output/scene.png"));
    t.end();
}
