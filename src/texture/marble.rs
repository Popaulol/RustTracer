use crate::color::Color;
use crate::noise::Perlin;
use crate::point3::Point3;
use crate::texture::{NoiseTexture, Texture};

pub struct MarbleTexture {
    scale: f64,
    noise: Perlin,
}

impl MarbleTexture {
    pub(crate) fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for MarbleTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let s = self.scale * p;

        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z() + 10.0 * self.noise.turb(&s)).sin())
    }
}
