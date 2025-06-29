use std::ops::{Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Mul<i32> for Vector {
    type Output = Self;

    // scalar multiplication by another type
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    // scalar multiplication by another type
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}
