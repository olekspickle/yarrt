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

    pub fn color(&self) -> Vec3 {
        let t = self.hit_sphere(Vec3::new(-0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).norm();
            return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }   
        let norm = self.direction().norm();
        let t = 0.5 * norm.y + 1.0;
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }

    /// We can read this as "any point p that satisfies this equation is on the sphere"
    /// t*t*dot(B,B) + 2*t*dot(B, A-C) + dot(A-C, A-C) - R*R = 0
    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> f32 {
        let oc: Vec3 = self.origin() - center;
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        (discriminant < 0.0)
            .then(|| -1.0)
            .unwrap_or_else(|| (-b - discriminant.sqrt()) / (2.0 * a))
    }
}
