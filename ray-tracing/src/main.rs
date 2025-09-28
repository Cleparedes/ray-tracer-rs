pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod utilities;
pub mod vec3;

use std::fs::{create_dir_all, File};
use std::io::Result;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() -> Result<()> {
    // Output
    create_dir_all("./output/")?;
    let mut image = File::create("./output/image.ppm")?;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));

    //Render
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.render(&mut world, &mut image)?;

    Ok(())
}
