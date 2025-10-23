use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::ops::Index;
use std::path::Path;
use image::{ImageDecoder, ImageReader};
use image::codecs::hdr::HdrDecoder;

#[derive(Clone)]
pub struct RTWImage {
    bytes_per_pixel: i32,
    fdata: Vec<f32>,
    bdata: Vec<u8>,
    image_width: i32,
    image_height: i32,
    bytes_per_scanline: i32,
}

impl RTWImage {
    pub fn new(image_filename: &str) -> Self {
        let _ = create_dir_all("./input/");
        let mut result = Self::default();
        let filename = format!("./input/{}", image_filename);
        if result.load(&filename) {
            return result
        }
        panic!("ERROR: Could not load image file");
    }

    pub fn load(&mut self, filename: &str) -> bool {
        // clear any previous data
        self.fdata.clear();
        self.bdata.clear();

        let path = Path::new(filename);

        // Handle Radiance HDR files (.hdr) which provide f32 samples
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case("hdr") {
                let file = match File::open(path) {
                    Ok(f) => f,
                    Err(_) => return false,
                };
                let reader = BufReader::new(file);
                let decoder = match HdrDecoder::new(reader) {
                    Ok(d) => d,
                    Err(_) => return false,
                };
                let (w, h) = decoder.dimensions();
                // total floats (RGB) and total bytes
                let total_floats = (w as usize) * (h as usize) * 3;
                let mut raw_bytes = vec![0u8; total_floats * std::mem::size_of::<f32>()];
                if decoder.read_image(&mut raw_bytes).is_err() {
                    return false;
                }

                // convert bytes -> f32 (native endian)
                self.fdata = Vec::with_capacity(total_floats);
                for i in 0..total_floats {
                    let off = i * 4;
                    let b = [raw_bytes[off], raw_bytes[off + 1], raw_bytes[off + 2], raw_bytes[off + 3]];
                    self.fdata.push(f32::from_ne_bytes(b));
                }

                self.image_width = w as i32;
                self.image_height = h as i32;
                self.bytes_per_pixel = 3;
                self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
                self.convert_to_bytes();
                return true;
            }
        }

        // Fallback: use ImageReader for other formats and convert to f32 [0..1]
        let reader = match ImageReader::open(filename) {
            Ok(r) => r,
            Err(_) => return false,
        };

        let reader = match reader.with_guessed_format() {
            Ok(r) => r,
            Err(_) => return false,
        };

        let dyn_img = match reader.decode() {
            Ok(i) => i,
            Err(_) => return false,
        };

        let rgb8 = dyn_img.to_rgb8(); // drop alpha if present and convert to 8-bit
        let (w, h) = (rgb8.width(), rgb8.height());
        let w_i = w as i32;
        let h_i = h as i32;
        self.image_width = w_i;
        self.image_height = h_i;
        self.bytes_per_pixel = 3;

        let total = (w_i * h_i * self.bytes_per_pixel) as usize;
        self.fdata = Vec::with_capacity(total);
        for p in rgb8.pixels() {
            // Map u8 0..255 to f32 0.0..1.0. Per your request gamma=1 (no gamma conversion).
            self.fdata.push(p[0] as f32 / 255.0);
            self.fdata.push(p[1] as f32 / 255.0);
            self.fdata.push(p[2] as f32 / 255.0);
        }

        self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;
        self.convert_to_bytes();
        true
    }

    pub fn width(&self) -> i32 {
        if self.fdata.is_empty() {
            return 0
        }
        self.image_width
    }

    pub fn height(&self) -> i32 {
        if self.fdata.is_empty() {
            return 0
        }
        self.image_height
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> usize {
        if self.bdata.is_empty() {
            return 0
        }
        let x: i32 = self.clamp(x, 0, self.image_width);
        let y: i32 = self.clamp(y, 0, self.image_height);

        return (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize;
    }

    fn clamp(&self, x: i32, low: i32, high: i32) -> i32 {
        if x < low {
            return low
        }
        if x < high {
            return x;
        }
        high - 1
    }

    fn float_to_byte(&self, value: f32) -> u8 {
        match value {
            ..=0.0 => 0,
            1.0.. => 255,
            _ => (256.0 * value) as u8,
        }
    }

    fn convert_to_bytes(&mut self) -> () {
        let total_bytes = (self.image_width * self.image_height * self.bytes_per_pixel) as usize;
        self.bdata = vec![0; total_bytes];
        for i in 0..total_bytes {
            self.bdata[i] = self.float_to_byte(self.fdata[i])
        }
    }
}

impl Default for RTWImage {
    fn default() -> Self {
        Self { 
            bytes_per_pixel: 3, 
            fdata: vec![], 
            bdata: vec![], 
            image_width: 0, 
            image_height: 0, 
            bytes_per_scanline: 0 
        }
    }
}

impl Index<usize> for RTWImage {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bdata[index]
    }
}