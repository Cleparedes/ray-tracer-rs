use crate::color::Color;
use crate::interval::Interval;
use crate::perlin::Perlin;
use crate::rtw_image::RTWImage;
use crate::vec3::Point3;

pub trait Texture {
    fn clone_box(&self) -> Box<dyn Texture>;
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl Clone for Box<dyn Texture> {

    fn clone(&self) -> Self {
        self.clone_box()
    }    
}

#[derive(Clone, Copy, Default)]
pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self::new_from_color(&Color::new(red, green, blue))
    }

    pub fn new_from_color(albedo: &Color) -> Self {
        Self {
            albedo: *albedo
        }
    }
}

impl Texture for SolidColor {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }

    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self::new_from_textures(
            scale, 
            Box::new(SolidColor::new_from_color(c1)), 
            Box::new(SolidColor::new_from_color(c2)),
        )
    }

    pub fn new_from_textures(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }

    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_floor = (self.inv_scale * p.x()).floor() as i32;
        let y_floor = (self.inv_scale * p.y()).floor() as i32;
        let z_floor = (self.inv_scale * p.z()).floor() as i32;
        let is_even: bool = (x_floor + y_floor + z_floor) % 2 == 0;
        if is_even {
            return self.even.value(u, v, p)
        }
        self.odd.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: RTWImage
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self { 
            image: RTWImage::new(filename)
        }
    }
}

impl Texture for ImageTexture {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }

    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(1.0, 0.0, 1.0)
        }
        let u: f64 = Interval::new(0.0, 1.0).clamp(u);
        let v: f64 = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel: usize = self.image.pixel_data(i, j);
        let color_scale: f64 = 1.0 / 255.0;
        Color::new(
            color_scale * self.image[pixel] as f64, 
            color_scale * self.image[pixel + 1] as f64, 
            color_scale * self.image[pixel + 2] as f64
        )
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }

    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) 
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}