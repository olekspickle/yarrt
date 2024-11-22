pub mod materials;
pub mod parallel;
pub mod scene;
pub mod utils;

use glam::Vec3;
use image::Rgb;
use rand::Rng;

use materials::BoxedMaterial;
use parallel::{ANTIALIASING, DEPTH, HEIGHT, WIDTH};
use scene::Camera;

/// Determine ray color
pub fn pixel_color(x: f32, y: f32, camera: &Camera, world: &Vec<BoxedSurface>) -> Rgb<u8> {
    let mut col = Vec3::ZERO;

    for _ in 0..ANTIALIASING {
        let rand = || rand::thread_rng().gen_range(0.0..1.0);
        
        // This two are pixel's relative coordinates on the screen
        let (u, v) = (((x + rand()) / WIDTH), ((y + rand()) / HEIGHT));
        let ray = camera.get_ray(u, v);
        col += ray.color(&world, DEPTH);
    }

    col /= ANTIALIASING as f32;
    col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
    Rgb([
        (col[0] * 255.99) as u8,
        (col[1] * 255.99) as u8,
        (col[2] * 255.99) as u8,
    ])
}

pub trait Vec3Ext<T> {
    /// Unit vector: v / v.length()
    fn unit_vec(&self) -> T;
}

impl Vec3Ext<Vec3> for Vec3 {
    fn unit_vec(&self) -> Vec3 {
        *self / self.length()
    }
}

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    /// Return ray in point t
    pub fn at(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }

    /// For ray calculate color in the given world - list of objects with coordinates
    pub fn color(&self, world: &Vec<BoxedSurface>, depth: i32) -> Vec3 {
        if let Some(hit) = world.hit(&self, 0.001, f32::MAX) {
            if let Some((attenuation, scattered)) = hit.material.scatter(self, &hit) {
                let col = scattered.color(world, depth);
                return attenuation * col;
            }
        }

        let norm = self.direction().unit_vec();
        let t = 0.5 * norm.y + 1.0;
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

#[derive(Clone)]
pub struct Hit<'a> {
    /// Ray parameter. Real number on a line AB, where
    /// A - ray origin
    /// B - ray direction:Surface
    /// --*(t=-1)---A(t=0)--->B(t=1)---*(t=2)---
    pub t: f32,
    /// 3D position on the line
    pub p: Vec3,
    /// Normal of the surface
    pub normal: Vec3,
    pub material: &'a BoxedMaterial,
}

impl<'a> Hit<'a> {
    pub fn new(t: f32, ray: &Ray, material: &'a BoxedMaterial, normal: Vec3) -> Self {
        let p = ray.at(t);

        Self {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub type BoxedSurface = Box<dyn Surface + Sync + Send + 'static>;
