use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::utilities::{INFINITY, degrees_to_radians};
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: &Point3, 
        normal: &Vec3, 
        material: Box<dyn Material>, 
        time: f64, 
        u: f64, v: f64, 
        front_face: bool) -> Self {

        Self {
            point: *point,
            normal: *normal,
            material: material,
            time,
            u,
            v,
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
            u: 0.0,
            v: 0.0,
            time: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: HittableClone {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> HittableClone for T where T: 'static + Hittable + Clone, {
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: &Vec3) -> Self {
        let bbox: AABB = object.bounding_box() + *offset;
        Self { 
            object, 
            offset: *offset, 
            bbox 
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let offset_ray = Ray::new(&(*ray.origin() - self.offset), ray.direction(), Some(ray.time()));
        if !self.object.hit(&offset_ray, interval, record) {
            return false
        }
        record.point += self.offset;
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, angle: f64) -> Self {
        let radians: f64 = degrees_to_radians(angle);
        let sin_theta: f64 = radians.sin();
        let cos_theta: f64 = radians.cos();
        let mut bbox: AABB = object.bounding_box();
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f64 = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y: f64 = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z: f64 = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let new_x: f64 = cos_theta * x + sin_theta * z;
                    let new_z: f64 = - sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        bbox = AABB::new_from_points(&min, &max);
        Self { 
            object, 
            sin_theta, 
            cos_theta, 
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        // Transform
        let origin = Point3::new(
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()), 
            ray.origin().y(),
            (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z()),
        );
        let direction = Vec3::new(
            (self.cos_theta * ray.direction().x()) - (self.sin_theta * ray.direction().z()), 
            ray.direction().y(),
            (self.sin_theta * ray.direction().x()) + (self.cos_theta * ray.direction().z()),
        );
        let rotated_ray = Ray::new(&origin, &direction, Some(ray.time()));
        // Check intersection
        if !self.object.hit(&rotated_ray, interval, record) {
            return false
        }
        // Transform
        record.point = Point3::new(
            (self.cos_theta * record.point.x()) + (self.sin_theta * record.point.z()), 
            record.point.y(), 
            (- self.sin_theta * record.point.x()) + (self.cos_theta * record.point.z()),
        );
        record.normal = Vec3::new(
            (self.cos_theta * record.normal.x()) + (self.sin_theta * record.normal.z()), 
            record.normal.y(), 
            (- self.sin_theta * record.normal.x()) + (self.cos_theta * record.normal.z()),
        );
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}