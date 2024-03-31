mod checker_texture;
mod image_texture;
mod marble;
mod noise_texture;
mod solid_color;

use crate::color::Color;
use crate::point3::Point3;

pub use checker_texture::CheckerTexture;
pub use image_texture::ImageTexture;
pub use marble::MarbleTexture;
pub use noise_texture::NoiseTexture;
pub use solid_color::SolidColor;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
