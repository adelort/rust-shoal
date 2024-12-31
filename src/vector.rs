use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub},
};

#[derive(Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, vector: Vector) -> Self::Output {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector { x: 0.0, y: 0.0 }, |acc, v| acc + v)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, vector: Vector) -> Self::Output {
        Vector {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
