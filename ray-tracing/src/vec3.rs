use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::interval::Interval;
use crate::utilities::random_double;

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let epsilon: f64 = 1e-8;
        f64::abs(self.x) < epsilon && f64::abs(self.y) < epsilon && f64::abs(self.z) < epsilon
    }
}

// fmt
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// ops
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self { 
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        1.0/factor * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, factor: f64) {
        *self *= 1.0/factor
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range")
        }
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range")
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(
            self.x * other.x,
            self.y * other.y, 
            self.z * other.z,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Vec3::new(
            factor * self.x, 
            factor * self.y, 
            factor * self.z,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        vector * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, factor: f64) {
        *self = Self { 
            x: factor * self.x,
            y: factor * self.y, 
            z: factor * self.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x, 
            y: -self.y, 
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

// utilities
pub fn dot(v: &Vec3, u: &Vec3) -> f64 {
    v.x() * u.x() + v.y() * u.y() + v.z() * u.z()
}

pub fn cross(v: &Vec3, u: &Vec3) -> Vec3 {
    Vec3::new(
        v.y() * u.z() - v.z() * u.y(),
        v.z() * u.x() - v.x() * u.z(),
        v.x() * u.y() - v.y() * u.x(),
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn random(interval_opt: Option<Interval>) -> Vec3 {
    Vec3::new(
        random_double(interval_opt), 
        random_double(interval_opt), 
        random_double(interval_opt))
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let point: Point3 = random(Some(Interval::new(-1.0, 1.0)));
        let len_sq: f64 = point.length_squared();
        if 1e-160 < len_sq && len_sq <= 1.0 {
            return point / f64::sqrt(len_sq)
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere
    } else {
        return -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}