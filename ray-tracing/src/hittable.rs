use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: &Point3, normal: &Vec3, time: f64, front_face: bool) -> Self {
        Self {
            point: *point,
            normal: *normal,
            time,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) -> () {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool;
}