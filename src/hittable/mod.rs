mod bvh;
mod hittable_list;
mod quad;
mod sphere;

use crate::hit_record::HitRecord;
use crate::interval::Interval;

use crate::aabb::Aabb;
use crate::Ray;
pub use bvh::BvhNode;
pub use hittable_list::HittableList;
pub use quad::Quad;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}
