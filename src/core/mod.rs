use rand::Rng;

use crate::core::ray::Ray;
use crate::vec3::{Vec3, Point3, Color};
use crate::objects::MaterialType;
use crate::objects::lambertian::Lambertian;
use crate::objects::metal::Metal;

pub mod ray;
pub mod camera;

pub trait HitTable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: MaterialType,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {    
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub fn ray_color<T>(ray: Ray, world: &T, depth: u32) -> Color 
        where T: HitTable {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec: HitRecord = Default::default();
    if world.hit(&ray, 0.0, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();

        if let MaterialType::Lambertian(l) = rec.material {
            if l.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, world, depth - 1);
            }
        } else if let MaterialType::Metal(m) = rec.material {
            if m.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


pub fn random_f64() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_range_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
