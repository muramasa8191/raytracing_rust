use crate::core::{HitTable, HitRecord};
use crate::core::ray::Ray;

pub struct HitTableList<T>
    where T: HitTable {
    objects: Vec<T>,
}

impl <T> HitTableList<T> 
    where T: HitTable {

    pub fn new() -> HitTableList<T> {
        HitTableList {
            objects: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl <T> HitTable for HitTableList<T> 
    where T: HitTable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut hit_rec = Default::default();

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut hit_rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                *rec = hit_rec;
            }
        }

        hit_anything
    }
}