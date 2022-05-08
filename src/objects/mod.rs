pub mod sphere;
pub mod hittable_list;
pub mod lambertian;
pub mod metal;

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    None,
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::None
    }
}
