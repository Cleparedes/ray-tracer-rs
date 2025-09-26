use crate::vec3::Vec3;
use std::fs::File;
use std::io::{Write, Result};

pub type Color = Vec3;

pub fn write_color(out: &mut File, pixel_color: &Color) -> Result<()> {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    let red: i32 = (255.999 * r) as i32;
    let green: i32 = (255.999 * g) as i32;
    let blue: i32 = (255.999 * b) as i32;

    writeln!(out, "{red} {green} {blue}")?;
    Ok(())
}