pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3, unit_vector};
use std::fs::{self, File};
use std::io::{Write, Result};

const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record: HitRecord = Default::default();
    if world.hit(ray, Interval::new(0.0, INFINITY), &mut record) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0))
    }

    let unit_direction = unit_vector(ray.direction());
    let step = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - step) * Color::new(1.0, 1.0, 1.0) 
        + step * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<()> {
    // Output
    fs::create_dir_all("./output/")?;
    let mut image = File::create("./output/image.ppm")?;

    // Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;

    // Image height
    let mut image_heigth: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
    image_heigth = if image_heigth < 1 { 1 } else { image_heigth };

    // World
    let mut world: HittableList = Default::default();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height 
        * ((IMAGE_WIDTH as f64) / (image_heigth as f64));
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport edge vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Viewport pixel vectors
    let pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
    let pixel_delta_v = viewport_v / (image_heigth as f64);

    // Pixel vector
    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left 
        + 0.5 * (pixel_delta_u + pixel_delta_v);

    //Render
    writeln!(&mut image, "P3")?;
    writeln!(&mut image, "{IMAGE_WIDTH} {image_heigth}")?;
    writeln!(&mut image, "255")?;

    for j in 0..image_heigth {
        print!("\rScanlines remaining: {} ", (image_heigth - j));

        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc 
                + (i as f64 * pixel_delta_u) 
                + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&ray, &world);
            write_color(&mut image, &pixel_color)?;
        }
    }

    println!("\rDone.                 ");
    Ok(())
}
