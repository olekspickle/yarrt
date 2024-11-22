pub mod materials;
pub mod scene;
pub mod utils;

use glam::Vec3;
use materials::Material;

/// For ray calculate color in the given world - list of objects with coordinates
pub fn color(ray: &Ray, world: &Vec<Box<dyn Surface>>, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0001, f32::MAX) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            let col: Vec3 = color(&scattered, world, depth + 1);
            return attenuation * col;
        }
    }

    let norm = ray.direction().unit_vec();
    let t = 0.5 * norm.y + 1.0;
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
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

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.a + t * self.b
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
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(
        t: f32,
        ray: &Ray,
        origin: Vec3,
        material: &'a dyn Material,
        face_normal: bool,
    ) -> Self {
        let p = ray.point_at(t);
        let normal = if face_normal { -p - origin } else { p - origin };

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
