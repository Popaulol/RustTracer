use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub(crate) fn new_with_color(albedo: Color) -> Lambertian {
        Self::new(Rc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
