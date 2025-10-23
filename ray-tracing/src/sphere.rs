use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utilities::PI;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Box<dyn Material>,
    bbox: AABB
}

impl Sphere {
    pub fn new(center: &Point3, center2: Option<&Point3>, radius: f64, material: Box<dyn Material>) -> Self {
        let direction: Vec3 = match center2 {
            Some(point) => *point - *center,
            None => Vec3::default(),
        };
        let center_ray = Ray::new(center, &direction, None);
        let rad = radius.max(0.0);
        let rvec = Vec3::new(rad, rad, rad);
        let box1 = AABB::new_from_points(&(center_ray.at(0.0) - rvec), &(center_ray.at(0.0) + rvec));
        let box2 = AABB::new_from_points(&(center_ray.at(1.0) - rvec), &(center_ray.at(1.0) + rvec));
        Self {
            center: center_ray,
            radius: rad,
            material,
            bbox: AABB::new_from_children(&box1, &box2),
        }
    }

    pub(crate) fn get_sphere_uv(&self, p: &Point3, u: &mut f64, v: &mut f64) -> () {
        let theta: f64 = (-p.y()).acos();
        let phi: f64 = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let current_center: Point3 = self.center.at(ray.time());
        let origin_center: Vec3 = current_center - *ray.origin();
        let a: f64 = ray.direction().length_squared();
        let h: f64 = dot(ray.direction(), &origin_center);
        let c: f64 = origin_center.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return false
        }

        // Nearest root in range
        let sqrt_discriminant: f64 = discriminant.sqrt();
        let mut root: f64 = (h - sqrt_discriminant) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !interval.surrounds(root) {
                return false
            }
        }

        record.time = root;
        record.point = ray.at(record.time);
        let outward_normal: Vec3 = (record.point - current_center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        self.get_sphere_uv(&outward_normal, &mut record.u, &mut record.v);
        record.material = self.material.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}