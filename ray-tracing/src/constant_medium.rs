use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{self, Interval};
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::utilities::{INFINITY, random_double};
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Box<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Box::new(Isotropic::new(albedo)),
        }
    }

    pub fn new_from_texture(boundary: Box<dyn Hittable>, density: f64, texture: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Box::new(Isotropic::new_from_texture(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();
        if !self.boundary.hit(ray, &interval::UNIVERSE, &mut rec1) {
            return false
        }
        if !self.boundary.hit(ray, &Interval::new(rec1.time + 0.0001, INFINITY), &mut rec2) {
            return false
        }
        if rec1.time < interval.min {
            rec1.time = interval.min;
        }
        if rec2.time > interval.max {
            rec2.time = interval.max;
        }
        if rec1.time >= rec2.time {
            return false
        }
        if rec1.time < 0.0 {
            rec1.time = 0.0;
        }
        let ray_length: f64 = ray.direction().length();
        let distance_inside_boundary: f64 = (rec2.time - rec1.time) * ray_length;
        let hit_distance: f64 = self.neg_inv_density * random_double(None).ln();
        if hit_distance > distance_inside_boundary {
            return false
        }
        record.time = rec1.time + hit_distance / ray_length;
        record.point = ray.at(record.time);
        record.normal = Vec3::new(1.0, 0.0, 0.0);
        record.front_face = true;
        record.material = self.phase_function.clone();
        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}