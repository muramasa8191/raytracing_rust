mod core;
mod objects;
mod ppm;
mod vec3;

use std::env;
use std::time::Instant;
use std::thread;
use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
    Mutex
};

use crate::core::camera::Camera;
use crate::core::HitTable;
use crate::objects::dielectric::Dielectric;
use crate::objects::hittable_list::HitTableList;
use crate::objects::lambertian::Lambertian;
use crate::objects::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::objects::MaterialType;
use crate::ppm::Image;
use crate::vec3::{Color, Point3, Vec3};

fn random_scene() -> HitTableList<Sphere> {
    let mut world = HitTableList::new();

    world.add(Sphere::new(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        MaterialType::Lambertian(Lambertian::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = core::random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * core::random_f64(),
                0.2,
                b as f64 + 0.9 * core::random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_vec3() * Vec3::random_vec3();
                    world.add(Sphere::new(
                        center.x(),
                        center.y(),
                        center.z(),
                        0.2,
                        MaterialType::Lambertian(Lambertian::new(
                            albedo.x(),
                            albedo.y(),
                            albedo.z(),
                        )),
                    ));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_vec3();
                    let fuzz = core::random_range_f64(0.0, 0.5);
                    world.add(Sphere::new(
                        center.x(),
                        center.y(),
                        center.z(),
                        0.2,
                        MaterialType::Metal(Metal::new(albedo.x(), albedo.y(), albedo.z(), fuzz)),
                    ));
                } else {
                    world.add(Sphere::new(
                        center.x(),
                        center.y(),
                        center.z(),
                        0.2,
                        MaterialType::Dielectric(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }
    world.add(Sphere::new(
        0.0,
        1.0,
        0.0,
        1.0,
        MaterialType::Dielectric(Dielectric::new(1.5)),
    ));
    world.add(Sphere::new(
        -4.0,
        1.0,
        0.0,
        1.0,
        MaterialType::Lambertian(Lambertian::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        4.0,
        1.0,
        0.0,
        1.0,
        MaterialType::Metal(Metal::new(0.7, 0.6, 0.5, 0.0)),
    ));

    world
}

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let basename = String::from("test.ppm");
    let filename = if args.len() >= 2 {
        &args[1]
    } else {
        &basename
    };
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 500;
    let max_depth = 50;

    // World
    let org_world = Arc::new(random_scene());

    // Camera
    let lookfrom_x = if args.len() >= 3 {
        args[2].parse::<f64>().unwrap()
    } else {
        13.0
    };
    let lookfrom_y = if args.len() >= 4 {
        args[3].parse::<f64>().unwrap()
    } else {
        2.0
    };
    let lookfrom_z = if args.len() >= 5 {
        args[4].parse::<f64>().unwrap()
    } else {
        3.0
    };

    let lookfrom = Point3::new(lookfrom_x, lookfrom_y, lookfrom_z);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Arc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    ));

    // Render

    let mut org_img = Arc::new(Mutex::new(Image::new(image_height, image_width)));
    let j_idx = Arc::new(AtomicI32::new((image_height-1) as i32));
    let mut threads = vec![];
    for _ in 0..16 {
        let atomic_j = j_idx.clone();
        let img = org_img.clone();
        let world = org_world.clone();
        let cam = camera.clone();
        let img_height = image_height;
        let img_width = image_width;
        let sp_per_pixel = samples_per_pixel;
        let max_dep = max_depth;
        threads.push(thread::spawn(move || {
            loop {
                let j = atomic_j.fetch_sub(1, Ordering::SeqCst);
                if j < 0 {
                    break;
                }
                for i in 0..img_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..sp_per_pixel {
                        let u = (i as f64 + core::random_f64()) / (img_width - 1) as f64;
                        let v = (j as f64 + core::random_f64()) / (img_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += core::ray_color(r, &world, max_dep);
                    }
                    img.lock().unwrap().set_color(img_height - (j as u32) - 1, i, &pixel_color, sp_per_pixel);
                }
                let left = start.elapsed().as_secs();
                let hour = left / (60 * 60);
                let min = left / 60 % 60;
                let sec = left % 60;
                println!(
                    "{}/{} done. {}:{}:{}.",
                    img_height - j as u32,
                    img_height,
                    hour,
                    min,
                    sec
                );
            }
        })
        );
    }
    for thread in threads {
        thread.join().unwrap();
    }
    org_img.lock().unwrap().save(filename).unwrap();
    let end = start.elapsed();
    println!(
        "finished in {}.{:03}",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
