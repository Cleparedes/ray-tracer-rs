use crate::vec3::{Point3, Vec3};

#[derive(Default, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, time: Option<f64>) -> Self {
        let t = match time {
            Some(tm) => tm,
            None => 0.0,
        };
        Self { 
            origin: *origin,
            direction: *direction,
            time: t,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.origin + time * self.direction
    }
}