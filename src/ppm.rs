use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::core::clamp;
use crate::vec3::Color;

#[derive(Debug, Clone)]
pub enum Pixel {
    Rgb([u8; 3]),
}

impl Pixel {
    pub fn zeros() -> Pixel {
        Pixel::Rgb([0, 0, 0])
    }
}

#[derive(Debug)]
pub struct Image {
    height: u32,
    width: u32,
    buffer: Vec<Pixel>,
}

impl Image {
    pub fn new(height: u32, width: u32) -> Image {
        Image {
            height,
            width,
            buffer: vec![Pixel::zeros(); (height * width) as usize],
        }
    }

    pub fn set_pixel(&mut self, y: u32, x: u32, rgb: Pixel) {
        self.buffer[(y * self.width + x) as usize] = rgb;
    }

    pub fn set_color(&mut self, y: u32, x: u32, color: &Color, samples_per_pixel: u32) {
        let mut r = color.x();
        let mut g = color.y();
        let mut b = color.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let rgb = Pixel::Rgb([
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8,
        ]);
        self.buffer[(y * self.width + x) as usize] = rgb;
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"P3\n")?;
        file.write_all(format!("{} {}\n", &self.width, &self.height).as_bytes())?;
        file.write_all(b"255\n")?;
        for pixel in &self.buffer {
            let Pixel::Rgb([r, g, b]) = pixel;
            file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }

        Ok(())
    }
}
