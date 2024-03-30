use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = Aabb::from_boxes(self.bbox, object.bounding_box());
        self.objects.push(object);
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

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
