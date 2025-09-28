pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

use std::fs::{create_dir_all, File};
use std::io::Result;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() -> Result<()> {
    // Output
    create_dir_all("./output/")?;
    let mut image = File::create("./output/image.ppm")?;

    // World
    let mut world = HittableList::default();
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_bubble = Box::new(Dielectric::new(1.0 / 1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    
    //Render
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.render(&mut world, &mut image)?;

    Ok(())
}
