use crate::{Hit, Ray, Vec3Ext};
use glam::Vec3;
use rand::Rng;

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

/// In the book this is the abstract class, but if we make it a trait
/// it will explode with generics all over the code. To follow KISS principle
/// we'll make it enum, aggregating logic for all materials
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

/// Represent simple materials that neither reflect nor refract
/// light rays
#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit.p + hit.normal + rand_in_unit_sphere();
        *scattered = Ray::new(hit.p, target - hit.p);
        // We could as well introduce some probability for scatter
        // let p = rand::thread_rng().gen_range(0.1..0.99);
        // *attenuation = self.albedo / p;
        *attenuation = self.albedo;
        return true;
    }
}

/// Represent reflective surfaces that reflect rays
#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Self::reflect(r_in.direction().norm(), hit.normal);
        *scattered = Ray::new(hit.p, reflected);
        // We could as well introduce some probability for scatter
        let p = rand::thread_rng().gen_range(0.1..0.99);
        *attenuation = self.albedo / p;
        // *attenuation = self.albedo;
        return scattered.direction().dot(hit.normal) > 0.0;
    }
}
