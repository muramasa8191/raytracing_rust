
mod core;
mod ppm;
mod vec3;
mod objects;

use std::time::Instant;

use crate::ppm::Image;
use crate::vec3::{Vec3, Point3};
use crate::core::ray::Ray;
use crate::objects::sphere::Sphere;
use crate::objects::hittable_list::HitTableList;

fn main() {
  let start = Instant::now();
  // Image
  let aspect_ratio: f64 = 16.0 / 9.0;
  let image_width: u32 = 400;
  let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

  // World
  let mut world = HitTableList::new();
  world.add(Sphere::new(0.0, 0.0, -1.0, 0.5));
  world.add(Sphere::new(0.0, -100.5, -1.0, 100.0));

  // Camera
  let viewport_height: f64 = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length: f64 = 1.0;

  let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
  let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
  let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
  let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

  // Render

  let mut img = Image::new(image_height, image_width);
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let u = (i as f64) / (image_width-1) as f64;
      let v = (j as f64) / (image_height-1) as f64;
      let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
      let pixel_color = core::ray_color(&r, &world);
      img.set_color(image_height-j-1, i, &pixel_color);
    }
  }
  img.save("test.ppm").unwrap();
  let end = start.elapsed();
  println!("finished in {}.{:03}", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
