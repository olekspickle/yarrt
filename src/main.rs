//! Implementation of ray tracing in one weekend renderer.
//!
//!

use glam::Vec3;
use image::{ImageBuffer, Rgb};
use rt_in_1_weekend::{utils, Ray};
use std::path::Path;

const DIM: u32 = 200;


fn main() {
    let (w, h) = (DIM as f32 * 2.0, DIM as f32);
    let mut buf = ImageBuffer::new(w as u32, h as u32);

    // Note: in the book Peter assumes that y is going up, but
    // image crate has y going down, and honestly I think its more intuitive
    let (lower_left, horiz, vert, origin) = (
        Vec3::new(-2.0, 1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::ZERO,
    );
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let (u, v) = ((x as f32 / w), (y as f32 / h));
        let ray = Ray::new(origin, lower_left + u * horiz + v * vert);
        let col = ray.color();
        *pixel = Rgb([
            (col[0] * 255.99) as u8,
            (col[1] * 255.99) as u8,
            (col[2] * 255.99) as u8,
        ]);
    }

    utils::save_image(buf, &Path::new("output/scene.png"));
}
