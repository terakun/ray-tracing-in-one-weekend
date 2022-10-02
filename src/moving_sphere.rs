use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    radius: f64,

    time0: f64,
    time1: f64,

    mat: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(c0: Point3, c1: Point3, r: f64, t0: f64, t1: f64, m: Rc<dyn Material>) -> Self {
        MovingSphere {
            center0: c0,
            center1: c1,
            radius: r,
            time0: t0,
            time1: t1,
            mat: m,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time-self.time0)/(self.time1-self.time0)) * (self.center1-self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
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
        let outward_normal = (p - self.center(r.time())) / self.radius;

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
