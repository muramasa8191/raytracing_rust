pub mod dielectric;
pub mod hittable_list;
pub mod lambertian;
pub mod metal;
pub mod sphere;

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
    None,
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::None
    }
}
