#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Complex (pub f64, pub f64);

use crate::element::RefInv;

use std::ops::{Add, Sub, Div, Mul, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt::Display;

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self(re, im)
    }
    pub fn normal2(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }
    pub fn normal(&self) -> f64 {
        self.normal2().sqrt()
    }
    pub fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
    pub fn adjoint(self) -> Self {
        Self(self.0, -self.1)
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0 * rhs.0 - self.1 * rhs.1;
        self.1 = self.0 * rhs.1 + self.1 * rhs.0;
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.mul(rhs.inv())
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        self.mul_assign(rhs.inv())
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}j", self.0, self.1)
    }
}

impl TryInto<Complex> for &str {
    type Error = ();
    fn try_into(self) -> Result<Complex, Self::Error> {
        if let Some(jidx) = self.find('j') {
            if let Some(aidx) = self.find('+') {
                if let (Ok(re), Ok(im)) = (&self[..aidx].parse(), &self[aidx+1..jidx].parse()) {
                    return Ok(Complex::new(*re, *im));
                }
            }
            if let Some(midx) = self.find('-') {
                if let (Ok(re), Ok(im)) = (&self[..midx].parse(), &self[midx+1..jidx].parse()) {
                    let mim: f64 = *im;
                    return Ok(Complex::new(*re, -mim));
                }
            }
            if let Ok(im) = &self[..jidx].parse() {
                return Ok(Complex::new(0.0, *im));
            }
        }
        return Err(());
    }
}

mod impl_linear {
    use crate::element::*;

    use super::Complex;

    impl AddZero for Complex {
        fn is_add_zero(&self) -> bool {
            self.normal() < 1e-6
        }
        fn add_zero() -> Self {
            Self(0.0, 0.0)
        }
    }
    impl MulZero for Complex {
        fn is_mul_zero(&self) -> bool {
            self.1.is_add_zero() && self.0.is_mul_zero()
        }
        fn mul_zero() -> Self {
            Self(1.0, 0.0)
        }
    }


    impl LinearElem for Complex {}
}
