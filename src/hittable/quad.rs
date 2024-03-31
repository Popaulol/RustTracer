use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit_vector();
        Self {
            q,
            u,
            v,
            mat,
            bbox: Aabb::from_points(q, q + &u + &v).pad(),
            normal,
            d: normal.dot(&q.into()),
            w: n / n.dot(&n),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(&r.origin().into())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpoint_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpoint_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpoint_vector));
        if !is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.material = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
        false
    } else {
        rec.u = a;
        rec.v = b;
        true
    }
}
