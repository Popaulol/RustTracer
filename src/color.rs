use crate::interval::Interval;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::ops;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Color {
    pub(crate) fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen::<f64>(),
            g: rng.gen::<f64>(),
            b: rng.gen::<f64>(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(min..max),
            g: rng.gen_range(min..max),
            b: rng.gen_range(min..max),
        }
    }
}

impl Color {
    pub(crate) fn r(&self) -> f64 {
        self.r
    }
    pub(crate) fn g(&self) -> f64 {
        self.g
    }
    pub(crate) fn b(&self) -> f64 {
        self.b
    }

    pub fn sample_scale(&mut self, samples: i32) {
        let scale = 1.0 / samples as f64;
        self.r *= scale;
        self.g *= scale;
        self.b *= scale;
    }

    pub fn gamma(&self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let intensity = Interval::new(0.0, 0.999);
        let ir = (256.0 * intensity.clamp(self.r)) as u8;
        let ig = (256.0 * intensity.clamp(self.g)) as u8;
        let ib = (256.0 * intensity.clamp(self.b)) as u8;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Self::Output::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
