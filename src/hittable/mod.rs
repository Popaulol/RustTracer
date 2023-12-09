mod sphere;

use crate::hit_record::HitRecord;
use crate::interval::Interval;

use crate::Ray;
pub use sphere::Sphere;
use std::rc::Rc;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}
