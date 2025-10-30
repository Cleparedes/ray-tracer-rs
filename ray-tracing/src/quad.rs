use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Box<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, material: Box<dyn Material>) -> Self {
        let n: Vec3 = cross(u, v);
        let normal: Vec3 = unit_vector(&n);
        let mut result = Self {
            q: *q,
            u: *u,
            v: *v,
            w: n / dot(&n, &n),
            material,
            bbox: AABB::default(),
            normal,
            d: dot(&normal, q)
        };
        result.set_bounding_box();
        result
    }

    pub fn set_bounding_box(&mut self) -> () {
        let bbox_diag1 = AABB::new_from_points(&self.q, &(self.q + self.u + self.v));
        let bbox_diag2 = AABB::new_from_points(&(self.q + self.u), &(self.q + self.v));
        self.bbox = AABB::new_from_children(&bbox_diag1, &bbox_diag2);
    }

    fn is_interior(&self, a: f64, b: f64, record: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false
        }
        record.u = a;
        record.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let denom: f64 = dot(&self.normal, ray.direction());
        // The ray is almost parallel to the plane
        if denom.abs() < 1e-8 {
            return false
        }

        // t is outside the ray interval
        let t = (self.d - dot(&self.normal, ray.origin())) / denom;
        if !interval.contains(t) {
            return false
        }
        
        // The hit point lies within the quadrilateral
        let intersection: Vec3 = ray.at(t);
        let planar_hitpt_vector: Vec3 = intersection - self.q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));
        if !self.is_interior(alpha, beta, record) {
            return false
        }

        record.time = t;
        record.point = intersection;
        record.material = self.material.clone();
        record.set_face_normal(ray, &self.normal);
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn make_box(a: &Point3, b: &Point3, material: Box<dyn Material>) -> Box<HittableList> {
    let mut sides = HittableList::default();
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));
    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());
    let red = Box::new(Lambertian::new(&Color ::new(0.65, 0.05, 0.05)));
    sides.add(Box::new(Quad::new(&Point3::new(min.x(), min.y(), max.z()), &dx, &dy, red.clone())));
    sides.add(Box::new(Quad::new(&Point3::new(max.x(), min.y(), max.z()), &-dy, &dy, material.clone())));
    sides.add(Box::new(Quad::new(&Point3::new(max.x(), min.y(), min.z()), &-dx, &dy, material.clone())));
    sides.add(Box::new(Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dz, &dy, material.clone())));
    sides.add(Box::new(Quad::new(&Point3::new(min.x(), max.y(), max.z()), &dx, &-dz, material.clone())));
    sides.add(Box::new(Quad::new(&Point3::new(min.x(), min.y(), min.z()), &dx, &dz, material.clone())));
    Box::new(sides)
}