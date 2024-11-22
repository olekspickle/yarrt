use crate::{ray_color, scene::Camera, BoxedSurface};
use image::Rgb;
use rayon::prelude::*;
use std::time::Instant;

pub const DIM: f32 = 200.0;
pub const HEIGHT: f32 = DIM;
pub const WIDTH: f32 = DIM * 2.0;
pub const ANTIALIASING: i32 = 50;
pub const DEPTH: i32 = 3;

/// The heavy lifting starts here.
/// This function allocates with size exactly if images pixels
/// to process them in chunks, here - lines(because chunk_size is
/// one line of pixels). This does improve render time in my laptop 4x,
/// but not solving stack overflow situation.
pub fn render(camera: Camera, world: Vec<BoxedSurface>) -> Vec<Rgb<u8>> {
    let (w, h) = (WIDTH as usize, HEIGHT as usize);

    let mut pixels = vec![Rgb([255, 255, 255]); w * h];
    let chunks: Vec<(usize, &mut [Rgb<u8>])> = pixels.chunks_mut(w).enumerate().collect();
    let start = Instant::now();
    chunks.into_par_iter().for_each(|(y, ch)| {
        let camera = camera.clone();
        let world = &world;
        ch.into_iter().enumerate().for_each(move |(x, p)| {
            let camera = camera.clone();
            *p = ray_color(x as f32, y as f32, &camera, &world);
        });
    });
    println!("Rendering time:{} s", start.elapsed().as_secs());
    pixels
}
