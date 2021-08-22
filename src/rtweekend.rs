use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random() -> f64 {
    rand::random::<f64>()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}