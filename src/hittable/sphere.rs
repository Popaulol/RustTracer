use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
    center_vec: Vec3,
    is_moving: bool,
    bbox: Aabb,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center,
            radius,
            material,
            center_vec: Vec3::default(),
            is_moving: false,
            bbox: Aabb::from_points(center - rvec, center + &rvec),
        }
    }

    pub(crate) fn new_moving(
        center: Point3,
        center_end: Point3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center - rvec, center + &rvec);
        let box2 = Aabb::from_points(center_end - rvec, center_end + &rvec);
        Self {
            center,
            radius,
            material,
            center_vec: center_end - center,
            is_moving: true,
            bbox: Aabb::from_boxes(box1, box2),
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center + &(time * self.center_vec)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.center(r.time())
        } else {
            self.center
        };
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = Some(self.material.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
