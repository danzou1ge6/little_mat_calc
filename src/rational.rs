use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A rational number
///
/// Rational(1, 2) = 1/2
///
/// 1 will always be (1, 1);
/// 0 will always be (0, 1);
/// and only `p` will be negative if `p/q` is negative
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rational(pub i32, pub i32);

fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        m
    } else {
        gcd(n, m % n)
    }
}

#[macro_export]
/// Create a [`Rational`] by calling its `new` method
macro_rules! rational {
    ($p:expr, $q:expr) => {
        $crate::Rational::new($p, $q)
    };
}

impl Rational {
    /// Create a new Rational
    ///
    /// Simplification will be run if using this method
    pub fn new(mut p: i32, mut q: i32) -> Self {
        if q < 0 {
            p = -p;
            q = -q
        };
        if q == 0 {
            panic!("Don't initialize a Rational with {}/0!", p);
        }

        let mut ret = Rational(p, q);
        ret.simplify();
        ret
    }

    /// Simplify the rational, on that it's valid, meaning q != 0 for p/q
    ///
    /// Does the following
    /// - if `p=0`, make `q=1`;
    /// - move the `-` to `p`
    /// - divide `p,q` by `gcd(p,q)`
    fn simplify(&mut self) {
        if self.0 == 0 {
            self.1 = 1;
            return;
        }
        if self.1 < 0 {
            self.0 = -self.0;
            self.1 = -self.1
        }

        let t = gcd(self.0.abs(), self.1.abs());
        self.0 /= t;
        self.1 /= t;
    }

    /// Rational(1, 2).inv() = Rational(2, 1)
    pub fn inv(self) -> Self {
        Self(self.1, self.0)
    }

    pub fn pow(self, n: u32) -> Self {
        Self(self.0.pow(n), self.1.pow(n))
    }
}

impl Add<&Self> for Rational {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let p = self.0 * rhs.1 + self.1 * rhs.0;
        let q = self.1 * rhs.1;
        let mut ret = Rational(p, q);
        ret.simplify();
        ret
    }
}

impl Sub<&Self> for Rational {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let p = self.0 * rhs.1 - self.1 * rhs.0;
        let q = self.1 * rhs.1;
        let mut ret = Rational(p, q);
        ret.simplify();
        ret
    }
}

impl Mul<&Self> for Rational {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut ret = Rational(self.0 * rhs.0, self.1 * rhs.1);
        ret.simplify();
        ret
    }
}

impl Div<&Self> for Rational {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        if rhs.0 == 0 {
            panic!("Devided by zero");
        }
        self.mul(&rhs.inv())
    }
}

impl From<Rational> for f64 {
    fn from(value: Rational) -> Self {
        let p: f64 = value.0.into();
        let q: f64 = value.1.into();
        p / q
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            write!(f, "0")
        } else if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

impl From<i32> for Rational {
    fn from(i: i32) -> Self {
        Self(i, 1)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.0 * other.1).partial_cmp(&(self.1 * other.0))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0 * other.1).cmp(&(self.1 * other.0))
    }
}

impl AddAssign<&Self> for Rational {
    fn add_assign(&mut self, rhs: &Self) {
        let v = *self + &rhs;
        self.0 = v.0;
        self.1 = v.1;
    }
}

impl SubAssign<&Self> for Rational {
    fn sub_assign(&mut self, rhs: &Self) {
        let v = *self - &rhs;
        self.0 = v.0;
        self.1 = v.1;
    }
}

impl MulAssign<&Self> for Rational {
    fn mul_assign(&mut self, rhs: &Self) {
        let v = *self * &rhs;
        self.0 = v.0;
        self.1 = v.1;
    }
}

impl DivAssign<&Self> for Rational {
    fn div_assign(&mut self, rhs: &Self) {
        let v = *self / rhs;
        self.0 = v.0;
        self.1 = v.1;
    }
}

use crate::element::*;

impl AddZero for Rational {
    fn add_zero() -> Self {
        Rational(0, 1)
    }
    fn is_add_zero(&self) -> bool {
        self.0 == 0
    }
}

impl MulZero for Rational {
    fn mul_zero() -> Self {
        Rational(1, 1)
    }
    fn is_mul_zero(&self) -> bool {
        self.0 == 1 && self.1 == 1
    }
}

impl LinearElem for Rational {}

#[cfg(test)]
mod test {

    #[test]
    fn test_new() {
        let a = rational!(6, 12);
        assert_eq!(a.0, 1);
        assert_eq!(a.1, 2);
    }

    #[test]
    fn test_add_pos() {
        assert_eq!(rational!(2, 3) + &rational!(5, 4), rational!(23, 12))
    }
    #[test]
    fn test_add_neg() {
        assert_eq!(rational!(-2, 3) + &rational!(5, 4), rational!(7, 12))
    }
    #[test]
    fn test_mul() {
        assert_eq!(rational!(2, 3) * &rational!(9, 7), rational!(6, 7))
    }
    #[test]
    fn test_div() {
        assert_eq!(rational!(2, 3) / &rational!(7, 9), rational!(6, 7))
    }
    #[test]
    fn test_gt() {
        assert_eq!(rational!(3, 2) > rational!(5, 4), true)
    }
    #[test]
    fn test_lt() {
        assert_eq!(rational!(3, 2) < rational!(5, 4), false)
    }
    #[test]
    fn test_eq() {
        assert_eq!(rational!(4, 9) == rational!(12, 27), true);
        assert_eq!(rational!(1, 2) == rational!(1, 2), true);
    }
}

mod from_str {
    use super::*;

    pub enum ParseError {
        ZeroDivision,
        NotARational,
    }

    impl TryInto<Rational> for &str {
        type Error = ParseError;
        fn try_into(self) -> Result<Rational, Self::Error> {
            if let Some(idx) = self.find('/') {
                let (p, mut q) = self.split_at(idx);
                q = &q[1..];
                if let (Ok(p), Ok(q)) = (p.parse(), q.parse()) {
                    if q == 0 { return Err(ParseError::ZeroDivision) }
                    return Ok(Rational::new(p, q));
                }
            }
            if let Ok(int) = self.parse::<i32>() {
                return Ok(Rational::from(int));
            }
            return Err(ParseError::NotARational);
        }
    }

}
pub use from_str::ParseError;
