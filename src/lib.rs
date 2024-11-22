pub mod utils;

use glam::Vec3;
use rand::Rng;

/// For ray calculate color in the given world - list of objects with coordinates
pub fn color<T: Surface>(ray: &Ray, world: &List<T>) -> Vec3 {
    let mut hit = Hit::default();
    if world.hit(ray, 0.0001, f32::MAX, &mut hit) {
        let target = hit.p + hit.normal + rand_in_unit_sphere();
        let new_ray = Ray::new(hit.p, target - hit.p);
        return 0.5 * color(&new_ray, world);
    } else {
        let norm = ray.direction().norm();
        let t = 0.5 * norm.y + 1.0;
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

/// Pick a random point in unit radius sphere centered at the origin.
/// We'll use the rejection algorithm:
/// 1. Peak a random point in the unit cube where x,y and z all in range -1..1
/// 2. If the point outside of the spere - reject it and try again
/// while we find the one that is inside of the sphere.
pub fn rand_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p =
            2.0 * Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ) - Vec3::ONE;
        if p.length_squared() >= 1.0 {
            break p;
        }
    }
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

#[derive(Default, Clone)]
pub struct Hit {
    /// Ray parameter. Real number on a line AB, where
    /// A - ray origin
    /// B - ray direction:Surface
    /// --*(t=-1)---A(t=0)--->B(t=1)---*(t=2)---
    pub t: f32,
    /// 3D position on the line
    pub p: Vec3,
    pub normal: Vec3,
}

impl Hit {
    pub fn update(&mut self, t: f32, ray: &Ray, origin: Vec3) {
        self.set_t(t);
        self.set_p(ray.point_at(self.t));
        self.set_normal(self.p - origin);
    }
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }
    pub fn set_p(&mut self, p: Vec3) {
        self.p = p;
    }
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
    pub fn new_unit() -> Self {
        Sphere {
            center: Vec3::ONE,
            radius: 5.0,
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit.update(temp, ray, self.center);
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit.update(temp, ray, self.center);
                return true;
            }
        }
        false
    }
}

pub struct List<T: Surface> {
    pub list: Vec<T>,
}

impl<T: Surface> List<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list }
    }
}

impl<T: Surface> Surface for List<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let mut temp_hit = Hit::default();
        let mut hit_any = false;
        let mut closest_so_far = t_max;
        for surface in self.list.iter() {
            let t = temp_hit.t;
            if surface.hit(ray, t_min, closest_so_far, &mut temp_hit) {
                hit_any = true;
                closest_so_far = t;
                *hit = temp_hit.clone();
            }
        }

        hit_any
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
