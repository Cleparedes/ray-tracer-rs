use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: &Point3, normal: &Vec3, material: Box<dyn Material>, time: f64, front_face: bool) -> Self {
        Self {
            point: *point,
            normal: *normal,
            material: material,
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

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Box::new(Lambertian::default()),
            time: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
    fn box_clone(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}