use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utilities::random_double;
use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3};

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

    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        let mut scatter_direction: Vec3 = record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal
        }
        *scattered = Ray::new(&record.point, &scatter_direction, Some(ray_in.time()));
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
        let mut reflected: Vec3 = reflect(ray_in.direction(), &record.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(&record.point, &reflected, Some(ray_in.time()));
        *attenuation = self.albedo;
        dot(scattered.direction(), &record.normal) > 0.0
    }
}

#[derive(Clone, Copy, Default)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { 
            refraction_index 
        }
    }

    // Schlick's approximation
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(*self)
    }

    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index: f64 = if record.front_face { 1.0 / self.refraction_index }
            else { self.refraction_index };
        let unit_direction: Vec3 = unit_vector(ray_in.direction());
        let cos_theta = 
            f64::min(dot(&(-unit_direction), &record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract: bool = refraction_index * sin_theta > 1.0;
        let direction: Vec3 = 
            if cannot_refract || self.reflectance(cos_theta, refraction_index) > random_double(None) { 
                reflect(&unit_direction, &record.normal) 
            } else { 
                refract(&unit_direction, &record.normal, refraction_index) 
            };
        *scattered = Ray::new(&record.point, &direction, Some(ray_in.time()));
        true
    }
}