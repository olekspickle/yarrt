pub mod utils;

use glam::Vec3;

pub trait Vec3Ext<T> {
    /// Unit vector: v / v.length()
    fn unit_vector(&self) -> T;
}

impl Vec3Ext<Vec3> for Vec3 {
    fn unit_vector(&self) -> Vec3 {
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
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * unit_direction.y + 1.0;
        (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
