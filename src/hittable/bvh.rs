use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut objects = objects.clone();

        let mut rng = thread_rng();
        let axis = rng.gen_range(0..3);

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: objects[start].clone().bounding_box(),
            }
        } else if object_span == 2 {
            if Self::box_compare(&objects[start], &objects[start + 1], axis) {
                Self {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bbox: Aabb::from_boxes(
                        objects[start].clone().bounding_box(),
                        objects[start + 1].clone().bounding_box(),
                    ),
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bbox: Aabb::from_boxes(
                        objects[start + 1].clone().bounding_box(),
                        objects[start].clone().bounding_box(),
                    ),
                }
            }
        } else {
            objects[start..end].sort_by(|a, b| {
                if Self::box_compare(a, b, axis) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mid = start + object_span / 2;

            let left = Rc::new(BvhNode::new(&objects, start, mid));
            let right = Rc::new(BvhNode::new(&objects, mid, end));

            Self {
                left: left.clone(),
                right: right.clone(),
                bbox: Aabb::from_boxes(left.bounding_box(), right.bounding_box()),
            }
        }
    }
    pub fn from_hittable_list(list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&list.objects, 0, len)
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: i32) -> bool {
        a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, mut ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
