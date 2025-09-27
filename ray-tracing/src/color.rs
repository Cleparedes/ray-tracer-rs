use crate::interval::Interval;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::{Write, Result};

pub type Color = Vec3;

pub fn write_color(out: &mut File, pixel_color: &Color) -> Result<()> {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Scale color components
    let intensity = Interval::new(0.0, 0.999);
    let rbyte: i32 = (256 as f64 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256 as f64 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256 as f64 * intensity.clamp(b)) as i32;

    writeln!(out, "{rbyte} {gbyte} {bbyte}")?;
    Ok(())
}