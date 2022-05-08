use crate::core::{HitRecord, Material};
use crate::core::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian { albedo: Color::new(r, g, b) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}
