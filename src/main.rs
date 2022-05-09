
mod core;
mod ppm;
mod vec3;
mod objects;

use std::time::Instant;

use crate::ppm::Image;
use crate::vec3::{Color, Point3, Vec3};
use crate::core::camera::Camera;
use crate::core::HitTable;
use crate::objects::sphere::Sphere;
use crate::objects::MaterialType;
use crate::objects::lambertian::Lambertian;
use crate::objects::metal::Metal;
use crate::objects::dielectric::Dielectric;
use crate::objects::hittable_list::HitTableList;

fn random_scene() -> HitTableList<Sphere> {
    let mut world = HitTableList::new();

    world.add(Sphere::new(0.0, -1000.0, 0.0, 1000.0, 
        MaterialType::Lambertian(Lambertian::new(0.5, 0.5, 0.5))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = core::random_f64();
            let center = Point3::new(a as f64 + 0.9 * core::random_f64(), 0.2, b as f64 + 0.9 * core::random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_vec3() * Vec3::random_vec3();
                    world.add(Sphere::new(center.x(), center.y(), center.z(), 0.2,
                              MaterialType::Lambertian(Lambertian::new(albedo.x(), albedo.y(), albedo.z()))));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_vec3();
                    let fuzz = core::random_range_f64(0.0, 0.5);
                    world.add(Sphere::new(center.x(), center.y(), center.z(), 0.2,
                              MaterialType::Metal(Metal::new(albedo.x(), albedo.y(), albedo.z(), fuzz))));
                } else {
                    world.add(Sphere::new(center.x(), center.y(), center.z(), 0.2,
                              MaterialType::Dielectric(Dielectric::new(1.5))));
                }
            }
        }
    }
    world.add(Sphere::new(0.0, 1.0, 0.0, 1.0, 
              MaterialType::Dielectric(Dielectric::new(1.5))));
    world.add(Sphere::new(-4.0, 1.0, 0.0, 1.0, 
              MaterialType::Lambertian(Lambertian::new(0.4, 0.2, 0.1))));
    world.add(Sphere::new(4.0, 1.0, 0.0, 1.0,
              MaterialType::Metal(Metal::new(0.7, 0.6, 0.5, 0.0))));

    world
}

fn main() {
    let start = Instant::now();
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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
