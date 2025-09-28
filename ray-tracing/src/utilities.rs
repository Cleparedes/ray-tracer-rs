use rand::Rng;

use crate::interval::Interval;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double(interval_opt: Option<Interval>) -> f64 {
    match interval_opt {
        None => rand::rng().random_range(0.0..1.0),
        Some(interval) => interval.min + (interval.max - interval.min) * random_double(None),
    }
}