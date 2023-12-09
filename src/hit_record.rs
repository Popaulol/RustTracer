use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
