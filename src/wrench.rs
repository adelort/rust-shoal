use std::{
    iter::Sum,
    ops::{Div, Mul},
};

use crate::vector::Vector;

pub struct Wrench {
    pub force: Vector,
    pub torque: f64,
}

impl Sum for Wrench {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Wrench {
                force: Vector { x: 0.0, y: 0.0 },
                torque: 0.,
            },
            |acc, w| Wrench {
                force: acc.force + w.force,
                torque: acc.torque + w.torque,
            },
        )
    }
}

impl Mul<f64> for Wrench {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Wrench {
            force: self.force * scalar,
            torque: self.torque * scalar,
        }
    }
}

impl Div<f64> for Wrench {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1. / scalar)
    }
}
