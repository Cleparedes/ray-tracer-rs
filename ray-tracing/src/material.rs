use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, unit_vector};

pub trait Material {
    fn clone_box(&self) -> Box<dyn Material>;

    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
        -> bool;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }    
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { 
            albedo 
        }
    }
}

impl Material for Lambertian {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }

    fn scatter(&self, _ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal
        }
        *scattered = Ray::new(&record.point, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone, Copy, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { 
            albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }

    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        let mut reflected = reflect(ray_in.direction(), &record.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(&record.point, &reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), &record.normal) > 0.0
    }
}