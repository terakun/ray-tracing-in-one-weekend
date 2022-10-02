use crate::vec3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, time: f64 ) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
            tm: time,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t : f64) -> Point3 {
        self.orig + t * self.dir
    }
}