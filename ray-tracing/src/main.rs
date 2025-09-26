pub mod color;
pub mod vec3;

use crate::color::color::write_color;
use crate::vec3::vec3::Vec3;
use std::fs::{self, File};
use std::io::{Write, Result};

fn main() -> Result<()> {
    fs::create_dir_all("./output/")?;
    let mut image = File::create("./output/image.ppm")?;

    let image_width: i32 = 256;
    let image_heigth: i32 = 256;

    writeln!(&mut image, "P3")?;
    writeln!(&mut image, "{image_width} {image_heigth}")?;
    writeln!(&mut image, "255")?;

    for j in 0..image_heigth {
        print!("\rScanlines remaining: {} ", (image_heigth - j));

        for i in 0..image_width {
            let mut pixel_color = Vec3::new(
                f64::from(i)/f64::from(image_width-1),
                f64::from(j)/f64::from(image_heigth-1),
                0.0,
            );

            write_color(&mut image, &mut pixel_color)?;
        }
    }

    println!("\rDone.                 ");
    Ok(())
}
