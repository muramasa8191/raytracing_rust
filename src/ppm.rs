use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
            buffer: vec![Pixel::zeros(); (height * width) as usize]
        }
    }

    pub fn set_pixel(&mut self, y: u32, x: u32, rgb: Pixel) {
        self.buffer[(y*self.width+x) as usize] = rgb;
    }

    pub fn set_color(&mut self, y: u32, x: u32, color: &Color) {
        let r: u8 = (255.999 * color.x()) as u8;
        let g: u8 = (255.999 * color.y()) as u8;
        let b: u8 = (255.999 * color.z()) as u8;
        self.buffer[(y*self.width+x) as usize] = Pixel::Rgb([r, g, b]);
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