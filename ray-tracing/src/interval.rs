use std::fmt::{Display, Formatter, Result};

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
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
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

pub const EMPTY: Interval = Interval { min: INFINITY, max: -INFINITY };
pub const UNIVERSE: Interval = Interval { min: -INFINITY, max: INFINITY };