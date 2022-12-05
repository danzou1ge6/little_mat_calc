#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Complex (pub f64, pub f64);

use crate::Rational;
use crate::element::{Inv, AddZero};

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
    pub fn re(&self) -> f64 { self.0 }
    pub fn im(&self) -> f64 { self.1 }
}

impl Add<&Self> for Complex {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign<&Self> for Complex {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl Sub<&Self> for Complex {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl SubAssign<&Self> for Complex {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<&Self> for Complex {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
    }
}
impl MulAssign<&Self> for Complex {
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 = self.0 * rhs.0 - self.1 * rhs.1;
        self.1 = self.0 * rhs.1 + self.1 * rhs.0;
    }
}
impl Div<&Self> for Complex {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self::Output {
        self.mul(&rhs.inv())
    }
}
impl DivAssign<&Self> for Complex {
    fn div_assign(&mut self, rhs: &Self) {
        self.mul_assign(&rhs.inv())
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1.is_add_zero() {
            write!(f, "{}", self.0)
        } else if self.0.is_add_zero() {
            write!(f, "{}j", self.1)
        } else {
            write!(f, "{}+{}j", self.0, self.1)
        }
    }
}

impl TryFrom<&str> for Complex {
    type Error = ();
    fn try_from(val: &str) -> Result<Complex, Self::Error> {
        if let Some(jidx) = val.find('j') {
            if let Some(aidx) = val.find('+') {
                if let (Ok(re), Ok(im)) = (&val[..aidx].parse(), &val[aidx+1..jidx].parse()) {
                    return Ok(Complex::new(*re, *im));
                }
            }
            if let Some(midx) = val.find('-') {
                if let (Ok(re), Ok(im)) = (&val[..midx].parse(), &val[midx+1..jidx].parse()) {
                    let mim: f64 = *im;
                    return Ok(Complex::new(*re, -mim));
                }
            }
            if let Ok(im) = &val[..jidx].parse() {
                return Ok(Complex::new(0.0, *im));
            }
        }
        if let Ok(flt) = val.parse() {
            return Ok(Complex::new(flt, 0.0));
        }
        return Err(());
    }

}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self(value, 0.0)
    }
}
impl From<Rational> for Complex {
    fn from(value: Rational) -> Self {
        f64::from(value).into()
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
