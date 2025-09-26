pub mod color {
    use crate::vec3::vec3::Vec3;
    use std::fs::File;
    use std::io::{Write, Result};

    pub fn write_color(out: &mut File, pixel_color: &mut Vec3) -> Result<()> {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let red = (255.999 * r) as i32;
        let green = (255.999 * g) as i32;
        let blue = (255.999 * b) as i32;

        writeln!(out, "{red} {green} {blue}")?;
        Ok(())
    }
}