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

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

pub type BoxedMaterial = Box<dyn Material + Send + Sync + 'static>;

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
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + rand_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        // We could as well introduce some probability for scatter
        // let p = rand::thread_rng().gen_range(0.1..0.99);
        // let attenuation = self.albedo / p;
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }
}

/// Represent reflective surfaces that reflect rays
#[derive(Clone)]
pub struct Reflective {
    albedo: Vec3,
}

impl Reflective {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
}

impl Material for Reflective {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = Self::reflect(r_in.direction().unit_vec(), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        // We could as well introduce some probability for scatter
        // let p = rand::thread_rng().gen_range(0.1..0.99);
        // *attenuation = self.albedo / p;
        let attenuation = self.albedo;
        if scattered.direction().dot(hit.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        None
    }
}

/// Represent refractive materials that change rays angles
/// I don't know why it was called dielectric...
/// FWIW water is definitely not dielectric.
#[derive(Clone)]
pub struct Refractive {
    refractive_index: f32,
}

impl Refractive {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(uv: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let dot = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dot * dot);
        if discriminant > 0.0 {
            return Some(ni_over_nt * (uv - n * dot) - n * discriminant.sqrt());
        }
        None
    }
}

impl Material for Refractive {
    /// It's interesting that the following code overflows stack sometimes...
    /// At least in my laptop.
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = Self::reflect(r_in.direction().unit_vec(), hit.normal);
        let (outward_normal, ni_over_nt) = if r_in.direction().dot(hit.normal) > 0.0 {
            (-hit.normal, self.refractive_index)
        } else {
            (hit.normal, 1.0 / self.refractive_index)
        };
        let refracted = Self::refract(r_in.direction(), outward_normal, ni_over_nt);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        if let Some(refracted) = refracted {
            return Some((attenuation, Ray::new(hit.p, refracted)));
        } else {
            // In this branch Peter returns false. 
            // It's important to keep the expected behavior
            return Some((attenuation, Ray::new(hit.p, reflected)));
        }
        // refracted
        //     .map(|refracted| Some((attenuation, Ray::new(hit.p, refracted))))
        //     .unwrap_or(Some((attenuation, Ray::new(hit.p, reflected))))
    }
}
