pub mod utils;

use glam::Vec3;

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

pub fn color<T: Surface>(ray: &Ray, world: &List<T>) -> Vec3 {
    let mut hit = Hit::default();
    if world.hit(ray, 0.0, f32::MAX, &mut hit) {
        return 0.5 * Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0);
    } else {
        let norm = ray.direction().norm();
        let t = 0.5 * norm.y + 1.0;
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
