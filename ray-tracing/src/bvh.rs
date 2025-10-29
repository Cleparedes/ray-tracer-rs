use std::cmp::Ordering;
use std::vec::Vec;

use crate::aabb::AABB;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::default();
        for object_index in start..end {
            bbox = AABB::new_from_children(&bbox, &objects[object_index].bounding_box())
        }

        let object_span: usize = end - start;
        let mut left: Box<dyn Hittable> = objects[start].clone();
        let mut right: Box<dyn Hittable> = objects[start].clone();
        match object_span {
            1 => {},
            2 => right = objects[start + 1].clone(),
            _ => {
                let axis: i32 = bbox.longest_axis();
                objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));
                let mid: usize = start + object_span / 2;
                left = Box::new(Self::new(objects, start, mid));
                right = Box::new(Self::new(objects, mid, end));
            }
        }
        
        Self { 
            left, 
            right, 
            bbox,
        }
    }

    pub fn new_from_hittable_list(list: &mut HittableList) -> Self {
        let end: usize = list.objects.len();
        Self::new(&mut list.objects, 0, end)
    }

    pub fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = *a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = *b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut crate::hittable::HitRecord) -> bool {
        if !self.bbox.hit(ray, interval) {
            return false
        }

        let hit_left: bool = self.left.hit(ray, interval, record);
        let hit_right: bool = self.right.hit(
            ray, 
            &Interval::new(
                interval.min, 
                if hit_left { record.time } else { interval.max }), 
            record);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}