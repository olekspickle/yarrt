use crate::{Hit, Ray, Vec3Ext};
use glam::Vec3;
use rand::Rng;

/// Pick a random point in unit radius sphere centered at the origin.
/// We'll use the rejection algorithm:
/// 1. Peak a random point in the unit cube where x,y and z all in range -1..1
/// 2. If the point outside of the spere - reject it and try again
/// while we find the one that is inside of the sphere.
pub fn rand_in_unit_sphere() -> Vec3 {
    let rand = || rand::thread_rng().gen_range(-1.0..1.0);
    loop {
        let p = 2.0 * Vec3::new(rand(), rand(), rand()) - Vec3::ONE;
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

    fn refract(uv: Vec3, n: Vec3, refraction_ratio: f32) -> Option<Vec3> {
        let dot = uv.dot(n);
        let discriminant = 1.0 - refraction_ratio.powi(2) * (1.0 - dot * dot);
        if discriminant > 0.0 {
            return Some(refraction_ratio * (uv - n * dot) - n * discriminant.sqrt());
        }
        None
    }

    /// Simplified Schlick algorithm for reflectance
    fn shlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    // fn refract_vec(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    //     let cos_theta = (-uv).dot(n).min(1.0);
    //     let r_out_perp = etai_over_etat * (uv + (cos_theta * n));
    //     let r_out_parallel = -(((1.0 - r_out_perp.length_squared()) as f32).abs().sqrt()) * n;
    //     r_out_perp + r_out_parallel
    // }
}

impl Material for Refractive {
    /// It's interesting that the following code overflows stack sometimes...
    /// At least in my laptop.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = Self::reflect(ray.direction().unit_vec(), hit.normal);
        let (outward_normal, refraction_ratio, cosine) = if ray.direction().dot(hit.normal) > 0.0 {
            (
                -hit.normal,
                self.refractive_index,
                self.refractive_index * ray.direction().dot(hit.normal) / ray.direction().length(),
            )
        } else {
            (
                hit.normal,
                1.0 / self.refractive_index,
                -ray.direction().dot(hit.normal) / ray.direction().length(),
            )
        };

        // // Borrowed solution from
        // // https://github.com/ndhansen/raytracer/blob/main/src/scene/materials/dielectric.rs
        // let unit_direction = ray.direction().unit_vec();
        // let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        // let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        // let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let attenuation = Vec3::splat(1.0);
        let random = rand::thread_rng().gen_range(0.0..1.0);
        let default_scattered = Ray::new(hit.p, reflected);
        let refracted = Self::refract(ray.direction(), outward_normal, refraction_ratio);
        let prob = refracted
            .map(|_| Self::shlick(cosine, self.refractive_index))
            .unwrap_or(1.0);
        let refracted = refracted.unwrap_or(Vec3::splat(0.5));
        if random < prob {
            return Some((attenuation, default_scattered));
        } else {
            return Some((attenuation, Ray::new(hit.p, refracted)));
        }

        // let reflectance = Self::shlick(cos_theta, refraction_ratio);
        // let direction = if cannot_refract || reflectance > random {
        //     // eprintln!("reflected with {} vs {} random! (cannot_refract: {})", reflectance, random_double, cannot_refract);
        //     Self::reflect(unit_direction, hit.normal)
        // } else {
        //     Self::refract_vec(unit_direction, hit.normal, refraction_ratio)
        // };
        // Some((attenuation, Ray::new(hit.p, direction)))
    }
}
