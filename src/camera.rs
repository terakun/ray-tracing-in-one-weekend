use crate::rtweekend;

use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // vfov: vertical field-of-view in degrees
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = rtweekend::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);
        let focal_length = 1.0;

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        }
    }
}
