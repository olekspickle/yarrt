pub mod materials;
pub mod utils;

use glam::Vec3;
use materials::Material;

/// For ray calculate color in the given world - list of objects with coordinates
pub fn color(ray: &Ray, world: &Vec<Box<dyn Surface>>, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.0001, f32::MAX) {
        let mut scattered = Ray::new(Vec3::ZERO, Vec3::splat(0.2));
        let mut attenuation: Vec3 = Vec3::ZERO;
        if depth < 50
            && hit
                .material
                .scatter(ray, &hit, &mut attenuation, &mut scattered)
        {
            let col: Vec3 = color(&scattered, world, depth + 1);
            return attenuation * col;
        }
    }

    let norm = ray.direction().norm();
    let t = 0.5 * norm.y + 1.0;
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}

pub trait Vec3Ext<T> {
    /// Unit vector: v / v.length()
    fn norm(&self) -> T;
}

impl Vec3Ext<Vec3> for Vec3 {
    fn norm(&self) -> Vec3 {
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
    pub fn new(t: f32, ray: &Ray, origin: Vec3, material: &'a dyn Material) -> Self {
        let p = ray.point_at(t);
        Self {
            t,
            p,
            normal: p - origin,
            material,
        }
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(Hit::new(temp, ray, self.center, &*self.material));
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(Hit::new(temp, ray, self.center, &*self.material));
            }
        }
        None
    }
}

impl Surface for Vec<Box<dyn Surface>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut temp_hit = None;
        let mut closest_so_far = t_max;
        for surface in self.iter() {
            if let Some(hit) = surface.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                temp_hit = Some(hit);
            }
        }

        temp_hit
    }
}

pub struct Camera {
    pub lower_left: Vec3,
    pub horiz: Vec3,
    pub vert: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + u * self.horiz + v * self.vert - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        // Note: in the book Peter assumes that y is going up, but
        // image crate pixel enumeration assumes has y going down,
        // and honestly I think it's much more intuitive
        Self {
            lower_left: Vec3::new(-2.0, 1.0, -1.0),
            horiz: Vec3::new(4.0, 0.0, 0.0),
            vert: Vec3::new(0.0, -2.0, 0.0),
            origin: Vec3::ZERO,
        }
    }
}
