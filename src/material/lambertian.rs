use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color) -> Lambertian {
        Self { albedo }
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

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
