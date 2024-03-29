use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub(crate) fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub(crate) fn origin(&self) -> Point3 {
        self.origin
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn time(&self) -> f64 {
        self.time
    }

    pub(crate) fn at(&self, t: f64) -> Point3 {
        &self.origin + t * self.direction
    }
}
