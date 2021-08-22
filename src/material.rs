
use crate::{hittable::HitRecord, vec3::{Color, Vec3}};

use super::ray::Ray;
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> ;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };

        let scattered = Ray{ orig: rec.p, dir: scatter_direction };
        let attenuation = self.albedo;
        Some((
            scattered,
            attenuation
        ))
    }
}

pub struct Metal {
    albedo: Color, 
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Metal { albedo: a, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);
        let scattered = Ray { orig: rec.p, dir: reflected + self.fuzz * Vec3::random_in_unit_sphere() };
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
 