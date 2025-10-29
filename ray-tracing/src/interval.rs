use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

use crate::utilities::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn new_from_children(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding: f64 = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, displacement: f64) -> Self::Output {
        Interval::new(self.min + displacement, self.max + displacement)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, interval: Interval) -> Self::Output {
        interval + self
    }
}

pub const EMPTY: Interval = Interval { min: INFINITY, max: -INFINITY };
pub const UNIVERSE: Interval = Interval { min: -INFINITY, max: INFINITY };