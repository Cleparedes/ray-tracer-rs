use std::fs::File;
use std::io::{Write, Result};

use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utilities::{random_double, INFINITY};
use crate::vec3::{random_unit_vector, unit_vector, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable, image: &mut File) -> Result<()> {
        self.initialize();

        writeln!(image, "P3")?;
        writeln!(image, "{} {}", self.image_width, self.image_height)?;
        writeln!(image, "255")?;

        for j in 0..(self.image_height) {
            print!("\rScanlines remaining: {} ", (self.image_height - j));
            for i in 0..(self.image_width) {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }

                write_color(image, &(self.pixel_samples_scale * pixel_color))?;
            }
        }

        println!("\rDone.                 ");
        Ok(())
    }

    fn initialize(&mut self) -> () {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        self.center = Point3::default();

        // Viewport dimensions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = 
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));
        
        // Viewport edge vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Viewport pixel vectors
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Pixel vector
        let viewport_upper_left = 
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 
            - viewport_v / 2.0;
        self.pixel00_loc = 
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = 
            self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) 
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double(None) - 0.5, random_double(None) - 0.5, 0.0)
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default()
        }

        let mut record  = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, INFINITY), &mut record) {
            let direction = record.normal + random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(&record.point, &direction), depth - 1, world)
        }

        let unit_direction = unit_vector(ray.direction());
        let step = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - step) * Color::new(1.0, 1.0, 1.0) + step * Color::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self { 
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 100,
            pixel_samples_scale: 0.1,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Point3::default(),
            pixel_delta_v: Point3::default(),
        }
    }    
}