use std::fmt::{Display, Formatter, Result};

use crate::interval;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self { 
            x: *x, 
            y: *y, 
            z: *z 
        }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        Self { 
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())), 
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())), 
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())), 
        }
    }

    pub fn new_from_children(a: &AABB, b: &AABB) -> Self {
        Self {
            x: Interval::new_from_children(&a.x, &b.x),
            y: Interval::new_from_children(&a.y, &b.y),
            z: Interval::new_from_children(&a.z, &b.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let ray_origin: &Point3 = ray.origin();
        let ray_direction: &Vec3 = ray.direction();
        let mut min = ray_t.min;
        let mut max = ray_t.max;
        for axis in 0..3 {
            let ax: &Interval = self.axis_interval(axis);
            let adinv: f64 = 1.0 / ray_direction[axis];
            let t0: f64 = (ax.min - ray_origin[axis]) * adinv;
            let t1: f64 = (ax.max - ray_origin[axis]) * adinv;
            if t0 < t1 {
                if min < t0 {
                    min = t0;
                }
                if t1 < max {
                    max = t1;
                }
            } else {
                if min < t1 {
                    min = t1;
                }
                if t0 < max {
                    max = t0;
                }
            }
            if max <= min {
                return false
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1
            }
        }
        2
    }
}

impl Display for AABB {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, y: {}, z:{}", self.x, self.y, self.z)
    }
}

pub const EMPTY: AABB = AABB { x: interval::EMPTY, y: interval::EMPTY, z: interval::EMPTY };
pub const UNIVERSE: AABB = AABB { x: interval::UNIVERSE, y: interval::UNIVERSE, z: interval::UNIVERSE };