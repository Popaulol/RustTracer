use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use std::mem::swap;

#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        }
    }

    pub fn from_boxes(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: box0.x.combined(box1.x),
            y: box0.y.combined(box1.y),
            z: box0.z.combined(box1.z),
        }
    }

    pub fn axis(&self, n: i32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let invD = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * invD;
            let mut t1 = (self.axis(a).max - orig) * invD;

            if invD < 0.0 {
                swap(&mut t0, &mut t1)
            }

            if t0 > ray_t.min {
                ray_t.min = t0
            };
            if t1 < ray_t.max {
                ray_t.max = t1
            };

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}


