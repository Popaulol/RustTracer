use std::ops;

use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Point3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Point3 {
    pub(crate) fn x(&self) -> f64 {
        self.x
    }
    pub(crate) fn y(&self) -> f64 {
        self.y
    }
    pub(crate) fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl ops::Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

impl ops::Add<&Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl ops::Sub<&Point3> for &Point3 {
    type Output = Vec3;

    fn sub(self, rhs: &Point3) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Point3> for &Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Self::Output {
        &self - rhs
    }
}
