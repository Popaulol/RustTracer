use crate::color::Color;
use crate::noise::Perlin;
use crate::point3::Point3;
use crate::texture::Texture;
use std::io::stderr;
use std::io::Write;
use std::ops::Mul;

pub struct NoiseTexture {
    scale: f64,
    noise: Perlin,
}

impl NoiseTexture {
    pub(crate) fn new(scale: f64) -> Self {
        Self {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let s = self.scale * p;

        Color::new(1.0, 1.0, 1.0) * self.noise.turb(&s)
    }
}
