use crate::{materials::Material, Hit, Ray, Surface};
use glam::Vec3;

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
        if discriminant < 0.0 {
            return None;
        }

        let d_root = discriminant.sqrt();
        if discriminant > 0.0 {
            let root = (-b - d_root) / a;
            if root < t_max && root > t_min {
                let face_normal = ray.direction().dot(self.center) < 0.0;
                return Some(Hit::new(
                    root,
                    ray,
                    self.center,
                    &*self.material,
                    face_normal,
                ));
            }
            let root = (-b + d_root) / a;
            if root < t_max && root > t_min {
                let face_normal = ray.direction().dot(self.center) < 0.0;
                return Some(Hit::new(
                    root,
                    ray,
                    self.center,
                    &*self.material,
                    face_normal,
                ));
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
