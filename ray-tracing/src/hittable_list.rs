use std::vec::Vec;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::{Interval};
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut list = HittableList::default();
        list.add(object);
        list
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) -> () {
        self.bbox = AABB::new_from_children(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = interval.max;

        for object in &self.objects {
            if object.hit(
                ray, 
                &Interval::new(interval.min, closest_so_far), 
                &mut temp_record) 
            {
                hit_anything = true;
                closest_so_far = temp_record.time;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}