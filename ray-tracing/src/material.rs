use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utilities::random_double;
use crate::vec3::{Point3, Vec3, dot, random_unit_vector, reflect, refract, unit_vector};

pub trait Material: MaterialClone {
    fn scatter(&self, _ray_in: &Ray, _record: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }

    fn emmited(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T where T: 'static + Material + Clone, {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self::new_from_texture(Box::new(SolidColor::new_from_color(albedo)))
    }

    pub fn new_from_texture(texture: Box<dyn Texture>) -> Self {
        Self {
            texture
        }
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self { 
            texture: Box::new(SolidColor::default()),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        let mut scatter_direction: Vec3 = record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal
        }
        *scattered = Ray::new(&record.point, &scatter_direction, Some(ray_in.time()));
        *attenuation = self.texture.value(record.u, record.v, &record.point);
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { 
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        let mut reflected: Vec3 = reflect(ray_in.direction(), &record.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(&record.point, &reflected, Some(ray_in.time()));
        *attenuation = self.albedo;
        dot(scattered.direction(), &record.normal) > 0.0
    }
}

#[derive(Clone)]
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
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) 
            -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index: f64 = if record.front_face { 1.0 / self.refraction_index }
            else { self.refraction_index };
        let unit_direction: Vec3 = unit_vector(ray_in.direction());
        let cos_theta: f64 = dot(&(-unit_direction), &record.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
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

#[derive(Clone)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(albedo: &Color) -> Self {
        Self::new_from_texture(Box::new(SolidColor::new_from_color(albedo)))
    }

    pub fn new_from_texture(texture: Box<dyn Texture>) -> Self {
        Self {
            texture
        }
    }
}

impl Material for DiffuseLight {
    fn emmited(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.texture.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic {
    texture: Box<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: &Color) -> Self {
        Self { 
            texture: Box::new(SolidColor::new_from_color(albedo))
        }
    }

    pub fn new_from_texture(texture: Box<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(&record.point, &random_unit_vector(), Some(ray_in.time()));
        *attenuation = self.texture.value(record.u, record.v, &record.point);
        true
    }
}