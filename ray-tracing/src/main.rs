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

use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, File};
use std::io::Result;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::utilities::random_double;
use crate::vec3::{random, Point3, Vec3};

fn main() -> Result<()> {
    // Output
    create_dir_all("./output/")?;
    let mut image = File::create("./output/image.ppm")?;

    // World
    let mut world = HittableList::default();

    let material_ground = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(
        Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), None, 1000.0, material_ground))
    );

    let bar = ProgressBar::new(484);
    bar.set_message("Generating objects...");
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));
    for a in -11..11 {
        for b in -11..11 {
            bar.inc(1);
            let choose_material: f64 = random_double(None);
            let center = Point3::new(
                a as f64 + 0.9 * random_double(None), 
                0.2, 
                b as f64 + 0.9 * random_double(None)
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_material {
                    0.0..0.8 => {
                        let albedo: Color = random(None) * random(None);
                        let sphere_material = Box::new(Lambertian::new(albedo));
                        let center2 = center + Vec3::new(
                            0.0, 
                            random_double(Some(Interval::new(0.0, 0.5))), 
                            0.0
                        );
                        world.add(Box::new(Sphere::new(&center, Some(&center2),0.2, sphere_material)));
                    },
                    0.8..0.95 => {
                        let albedo: Color = random(Some(Interval::new(0.5, 1.0)));
                        let fuzz: f64 = random_double(Some(Interval::new(0.0, 0.5)));
                        let sphere_material = Box::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(&center, None, 0.2, sphere_material)));
                    },
                    _ => {
                        let sphere_material = Box::new(Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(&center, None, 0.2, sphere_material)));
                    }
                }
            }
        }
    }
    
    let glass_material = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 1.0, 0.0), None, 1.0, glass_material)));

    let diffuse_material = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), None,1.0, diffuse_material)));

    let metal_material = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), None, 1.0, metal_material)));
    
    bar.set_message("Generating objects: Done.");
    bar.finish();
    
    //Render
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vertical_view_angle = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::default();
    camera.view_up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.render(&mut world, &mut image)?;

    Ok(())
}
