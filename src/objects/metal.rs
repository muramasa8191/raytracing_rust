use crate::core::ray::Ray;
use crate::core::{HitRecord, Material};
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, f: f64) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal {
            albedo: Color::new(r, g, b),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
