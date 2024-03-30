use crate::color::Color;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::texture::Texture;
use image::{open, ImageResult, RgbImage};

pub struct ImageTexture {
    image: Option<RgbImage>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = open(filename);
        if let Ok(image) = image {
            Self {
                image: Some(image.into_rgb8()),
            }
        } else {
            Self { image: None }
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if let Some(image) = self.image.as_ref() {
            let interval = Interval::new(0.0, 1.0);

            let u = interval.clamp(u);
            let v = 1.0 - interval.clamp(v);

            let i = (u * image.width() as f64) as u32;
            let j = (v * image.height() as f64) as u32;

            let pixel = image.get_pixel(i, j);

            (*pixel).into()
        } else {
            Color::new(0.0, 1.0, 1.0)
        }
    }
}
