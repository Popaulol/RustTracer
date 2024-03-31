#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn combined(&self, other: Interval) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

impl Interval {
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}
