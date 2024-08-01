use std::ops::{AddAssign, Div, Mul, Neg};

use num::{One, Zero};

use crate::numeric::Numeric;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct ComplexNumber {
    real: f64,
    imag: f64,
}

impl Numeric for ComplexNumber {
    fn magnitude(self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

impl Into<f64> for ComplexNumber {
    fn into(self) -> f64 {
        self.magnitude()
    }
}

impl Zero for ComplexNumber {
    fn zero() -> Self {
        ComplexNumber { real: 0., imag: 0. }
    }

    fn is_zero(&self) -> bool {
        self.real == 0. && self.imag == 0.
    }
}

impl One for ComplexNumber {
    fn one() -> Self {
        ComplexNumber { real: 1., imag: 0. }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub for ComplexNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        ComplexNumber {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Neg for ComplexNumber {
    type Output = Self;

    fn neg(self) -> Self {
        ComplexNumber {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denominator = other.real * other.real + other.imag * other.imag;
        let real = (self.real * other.real + self.imag * other.imag) / denominator;
        let imag = (self.imag * other.real - self.real * other.imag) / denominator;
        ComplexNumber { real, imag }
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        ComplexNumber { real, imag }
    }
}
