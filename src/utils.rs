use image::{self, ImageBuffer, ImageFormat, Pixel, Rgb};
use std::{path::Path, time::Instant};
use termion::{color, style};

pub struct Timer(Instant);

impl Timer {
    pub fn new() -> Self {
        Self(Instant::now())
    }
    pub fn end(&self) {
        println!("Time taken: {:?}", self.0.elapsed());
    }
}

pub fn save_image(img: ImageBuffer<Rgb<u8>, Vec<u8>>, p: &Path) {
    print_italic(&format!("saving as {:?}...", p));
    match img.save_with_format(p, ImageFormat::Png) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}

pub fn print_green(s: &str) {
    println!("{}{s}{}", color::Fg(color::Green), color::Fg(color::Reset))
}

pub fn print_italic(s: &str) {
    print!("{}{s}{}", style::Italic, style::Reset);
}

pub struct Image<P: Pixel>(ImageBuffer<P, Vec<u8>>);

impl<P: Pixel> Image<P> {
    pub fn new(buf: ImageBuffer<P, Vec<u8>>) -> Self {
        Self(buf)
    }
}

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for Image<Rgb<u8>> {
    fn from(buf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        Self(buf)
    }
}

impl From<Image<Rgb<u8>>> for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn from(image: Image<Rgb<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        image.0
    }
}

impl<P: Pixel> Image<P> {
    /// Get reference to the inner image buffer
    pub fn buf(&self) -> &ImageBuffer<P, Vec<u8>> {
        &self.0
    }

    /// Get inner image buffer as mutable
    pub fn buf_mut(&mut self) -> &mut ImageBuffer<P, Vec<u8>> {
        &mut self.0
    }
}
