pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl Interval {
    fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
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

const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);
