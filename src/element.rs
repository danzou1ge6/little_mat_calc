use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

/// Meaning there exists `0` satisfying for any `a` eixsts `b` s.t. `0 = a + b`
pub trait AddZero {
    fn add_zero() -> Self;
    fn is_add_zero(&self) -> bool;
}

/// Meaning there exists `1` satisfying for any `a` exists `b` s.t. `1 = a*b`
pub trait MulZero {
    fn mul_zero() -> Self;
    fn is_mul_zero(&self) -> bool;
}

/// Species how `1/a` is calculated given reference to `a`
pub trait Inv {
    fn inv(self) -> Self;
}
impl<T> Inv for T
where
    T: Copy + for<'a> Div<&'a Self, Output = T> + MulZero,
{
    fn inv(self) -> Self {
        Self::mul_zero() / &self
    }
}

pub trait LinearElem:
    Sized
    + Clone
    + AddZero
    + MulZero
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + PartialEq
    + std::fmt::Display
    + std::fmt::Debug
{
}

/// Implements [`LinearElem`] for some primitives
mod implementations {
    use super::*;
    mod impl_f64 {
        use super::*;

        impl AddZero for f64 {
            fn add_zero() -> Self {
                0.0
            }
            fn is_add_zero(&self) -> bool {
                self.abs() <= 1e-6
            }
        }
        impl MulZero for f64 {
            fn mul_zero() -> Self {
                1.0
            }
            fn is_mul_zero(&self) -> bool {
                (self - 1.0).abs() <= 1e-6
            }
        }
        impl LinearElem for f64 {}
    }

    mod impl_f32 {
        use super::*;

        impl AddZero for f32 {
            fn add_zero() -> Self {
                0.0
            }
            fn is_add_zero(&self) -> bool {
                self.abs() <= 1e-6
            }
        }
        impl MulZero for f32 {
            fn mul_zero() -> Self {
                1.0
            }
            fn is_mul_zero(&self) -> bool {
                (self - 1.0).abs() <= 1e-6
            }
        }
        impl LinearElem for f32 {}
    }

    mod impl_i64 {
        use super::*;

        impl AddZero for i64 {
            fn add_zero() -> Self {
                0
            }
            fn is_add_zero(&self) -> bool {
                *self == 0
            }
        }
        impl MulZero for i64 {
            fn mul_zero() -> Self {
                1
            }
            fn is_mul_zero(&self) -> bool {
                *self == 1
            }
        }
        impl LinearElem for i64 {}
    }
    mod impl_i32 {
        use super::*;

        impl AddZero for i32 {
            fn add_zero() -> Self {
                0
            }
            fn is_add_zero(&self) -> bool {
                *self == 0
            }
        }
        impl MulZero for i32 {
            fn mul_zero() -> Self {
                1
            }
            fn is_mul_zero(&self) -> bool {
                *self == 1
            }
        }
        impl LinearElem for i32 {}
    }
}
