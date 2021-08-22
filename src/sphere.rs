use super::vec3::Point3;
use super::vec3::Vec3;

use super::ray::Ray;
use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3, 
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        } 
        let sqrtd = discriminant.sqrt();

        let root = (-half_b-sqrtd)/a;
        if root < t_min || t_max < root {
            let root = (-half_b+sqrtd)/a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p-self.center) / self.radius;

        let mut rec = HitRecord {
            p: p, 
            normal: outward_normal,
            t: t,
            front_face: false,
        };
        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }
}