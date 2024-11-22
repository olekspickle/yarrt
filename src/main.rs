//! Implementation of ray tracing in one weekend renderer.
//!
//!

use glam::Vec3;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use rayon::prelude::*;
use rt_in_1_weekend::{
    color,
    materials::{Lambertian, Reflective, Refractive},
    parallel::{RNG, WIDTH, WORLD},
    scene::{Camera, Sphere},
    utils, Surface,
};
use std::{cell::Cell, path::Path};

const DEPTH: i32 = 3;

fn world<'a>() -> Vec<Box<dyn Surface>> {
    let mut world: Vec<Box<dyn Surface>> = vec![];

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
    // ns is just step count for antialiasing
    let (w, h, ns) = (WIDTH, HEIGHT, ANTIALIASING);
    let mut image = Image::new(w as u32, h as u32);

    // let world: Vec<Box<dyn Surface>> = world();
    WORLD.set(world());
    let camera = Camera::default();

    let mut rng = rand::thread_rng();
    // for (x, y, pixel) in buf.enumerate_pixels_mut() {
    //     let mut col = Vec3::ZERO;
    //     let (x, y) = (x as f32, y as f32);

    //     for _ in 0..ns {
    //         let rand = rng.gen_range(0.0..1.0);

    //         // This two are pixel's relative coordinates on the screen
    //         let (u, v) = (((x + rand) / w), ((y + rand) / h));
    //         let ray = camera.get_ray(u, v);
    //         // let p = ray.point_at(2.0);
    //         col += color(&ray, &world, DEPTH);
    //     }

    //     col /= ns as f32;
    //     col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
    //     *pixel = Rgb([
    //         (col[0] * 255.99) as u8,
    //         (col[1] * 255.99) as u8,
    //         (col[2] * 255.99) as u8,
    //     ]);
    // }

    utils::save_image(buf, &Path::new("output/scene.png"));
    t.end();
}
