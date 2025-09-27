use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: *center,
            radius: f64::max(0.0, radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let origin: Point3 = self.center - *ray.origin();
        let a: f64 = ray.direction().length_squared();
        let h: f64 = dot(ray.direction(), &origin);
        let c: f64 = dot(&origin, &origin) - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return false
        }

        // Nearest root in range
        let sqrt_discriminant: f64 = f64::sqrt(discriminant);
        let mut root: f64 = (h - sqrt_discriminant) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !interval.surrounds(root) {
                return false
            }
        }

        record.time = root;
        record.point = ray.at(record.time);
        let outward_normal: Vec3 = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}