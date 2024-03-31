use std::ops;
use std::ops::{Mul, MulAssign};

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
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Point3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
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

impl ops::Index<i32> for Point3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Invalid Index on Point3")
            }
        }
    }
}

impl From<Point3> for Vec3 {
    fn from(val: Point3) -> Self {
        Vec3::new(val.x, val.y, val.z)
    }
}

impl Mul<&Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: &Point3) -> Self::Output {
        Point3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
