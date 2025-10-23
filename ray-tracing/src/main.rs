pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rtw_image;
pub mod sphere;
pub mod texture;
pub mod utilities;
pub mod vec3;

use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, File};
use std::io::Result;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::utilities::random_double;
use crate::vec3::{random, Point3, Vec3};

fn bouncing_spheres() -> Result<()> {
    // Output
    let mut image = File::create("./output/bouncing_spheres.ppm")?;

    let bar = ProgressBar::new(485);
    bar.set_message("Generating objects...");
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));

    // World
    let mut world = HittableList::default();

    let checker = CheckerTexture::new(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
    let ground_material = Box::new(Lambertian::new_from_texture(Box::new(checker)));
    world.add(
        Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), None, 1000.0, ground_material))
    );

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
                        let sphere_material = Box::new(Lambertian::new(&albedo));
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

    let diffuse_material = Box::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), None,1.0, diffuse_material)));

    let metal_material = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(&Point3::new(4.0, 1.0, 0.0), None, 1.0, metal_material)));
    
    world = HittableList::new(Box::new(BVHNode::new_from_hittable_list(&mut world)));

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

fn checkered_spheres() -> Result<()> {
    // Output
    let mut image = File::create("./output/checkered_spheres.ppm")?;

    let bar = ProgressBar::new(2);
    bar.set_message("Generating objects...");
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));

    // World
    let mut world = HittableList::default();

    let checker = CheckerTexture::new(0.32, &Color::new(0.2, 0.3, 0.1), &Color::new(0.9, 0.9, 0.9));
    let checker_material = Box::new(Lambertian::new_from_texture(Box::new(checker)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -10.0, 0.0), None, 10.0, checker_material.clone())));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 10.0, 0.0), None, 10.0, checker_material.clone())));

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
    camera.defocus_angle = 0.0;
    camera.render(&mut world, &mut image)?;

    Ok(())
}

fn earth() -> Result<()> {
    // Output
    let mut image = File::create("./output/earth.ppm")?;

    let bar = ProgressBar::new(1);
    bar.set_message("Generating objects...");
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));

    // World
    let mut world = HittableList::default();

    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Box::new(Lambertian::new_from_texture(Box::new(earth_texture)));
    world.add(Box::new(Sphere::new(&Point3::default(), None, 2.0, earth_surface)));

    bar.set_message("Generating objects: Done.");
    bar.finish();
    
    //Render
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vertical_view_angle = 20.0;
    camera.look_from = Point3::new(0.0, 0.0, 12.0);
    camera.look_at = Point3::default();
    camera.view_up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.render(&mut world, &mut image)?;

    Ok(())
}

fn perlin_spheres() -> Result<()> {
    // Output
    let mut image = File::create("./output/perlin_spheres.ppm")?;

    let bar = ProgressBar::new(2);
    bar.set_message("Generating objects...");
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-"));

    // World
    let mut world = HittableList::default();

    let pertext = NoiseTexture::new(4.0);
    let pertext_material = Box::new(Lambertian::new_from_texture(Box::new(pertext)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), None, 1000.0, pertext_material.clone())));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 2.0, 0.0), None, 2.0, pertext_material.clone())));

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
    camera.defocus_angle = 0.0;
    camera.render(&mut world, &mut image)?;

    Ok(())
}


fn main() -> Result<()> {
    let _ = create_dir_all("./output/")?;

    match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => perlin_spheres(),
    }
}