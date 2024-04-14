use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use std::rc::Rc;

pub struct DiffuseLight {
    tex: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub(crate) fn new(tex: Rc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(emit: Color) -> Self {
        Self::new(Rc::new(SolidColor::new(emit)))
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        return false;
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
