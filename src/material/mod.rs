mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::point3::Point3;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
