use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Rc<dyn Material>) -> Self {
        Sphere {
            center: c,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        let root = if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            } else {
                root
            }
        } else {
            root
        };

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord {
            p: p,
            normal: outward_normal,
            mat: self.mat.clone(),
            t: root,
            front_face: false,
        };
        rec.set_face_normal(&r, &outward_normal);

        Some(rec)
    }
}
