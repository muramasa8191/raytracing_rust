use crate::core::ray::Ray;
use crate::core::{HitRecord, HitTable};
use crate::objects::MaterialType;
use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: MaterialType,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, material: MaterialType) -> Self {
        Sphere {
            center: Point3::new(x, y, z),
            radius,
            material,
        }
    }
}

impl HitTable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = ((-half_b) - sqrtd) / a;
        if root < t_min || t_max < root {
            root = ((-half_b) + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.material = self.material;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);

        true
    }
}
