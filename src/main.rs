
mod core;
mod ppm;
mod vec3;
mod objects;

use std::time::Instant;

use crate::ppm::Image;
use crate::vec3::{Color, Point3, Vec3};
use crate::core::camera::Camera;
use crate::objects::sphere::Sphere;
use crate::objects::MaterialType;
use crate::objects::lambertian::Lambertian;
use crate::objects::metal::Metal;
use crate::objects::dielectric::Dielectric;
use crate::objects::hittable_list::HitTableList;

fn main() {
    let start = Instant::now();
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World
    let mut world = HitTableList::new();
    world.add(Sphere::new(0.0, -100.5, -1.0, 100.0, 
        MaterialType::Lambertian(Lambertian::new(0.8, 0.8, 0.0))));
    world.add(Sphere::new(0.0, 0.0, -1.0, 0.5,
        MaterialType::Lambertian(Lambertian::new(0.1, 0.2, 0.5))));
    world.add(Sphere::new(-1.0, 0.0, -1.0, 0.5,
        MaterialType::Dielectric(Dielectric::new(1.5))));
    world.add(Sphere::new(-1.0, 0.0, -1.0, -0.45,
        MaterialType::Dielectric(Dielectric::new(1.5))));
    world.add(Sphere::new(1.0, 0.0, -1.0, 0.5,
        MaterialType::Metal(Metal::new(0.8, 0.6, 0.2, 0.0))));
    // Camera
    let cam = Camera::new(Point3::new(-2.0, 2.0, -2.0), Point3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0),
                          30.0, 16.0 / 9.0);

    // Render

    let mut img = Image::new(image_height, image_width);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                // print!("({}, {})", j, i);
                let u = (i as f64 + core::random_f64()) / (image_width-1) as f64;
                let v = (j as f64 + core::random_f64()) / (image_height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += core::ray_color(r, &world, max_depth);
            }
            img.set_color(image_height-j-1, i, &pixel_color, samples_per_pixel);
        }
        println!("{}/{} done.", image_height - j, image_height);
    }
    img.save("test.ppm").unwrap();
    let end = start.elapsed();
    println!("finished in {}.{:03}", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
