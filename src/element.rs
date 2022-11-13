use std::ops::{AddAssign, SubAssign, MulAssign, Mul, Add, Sub, Div};


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
pub trait RefInv {
    fn inv(&self) -> Self;
}
impl<T> RefInv for T where T: Copy + Div<Output=T> + MulZero {
    fn inv(&self) -> Self { Self::mul_zero() / *self }
}

/// Species how to tell if `a == b` given references to them
pub trait RefEq {
    fn ref_eq(&self, rhs: &Self) -> bool;
}
impl<T> RefEq for T where T: AddZero + RefSub<Output=Self> {
    fn ref_eq(&self, rhs: &Self) -> bool {
        (self.ref_sub(rhs)).is_add_zero()
    }
}

/// Species how to substract `b` from `a` given their references
/// 
/// Following are the same
pub trait RefMulAssign {
    fn ref_mul_assign(&mut self, rhs: &Self);
}
impl<T> RefMulAssign for T where T: Copy + MulAssign {
    fn ref_mul_assign(&mut self, rhs: &Self) { *self *= *rhs; }
}
pub trait RefAddAssign {
    fn ref_add_assign(&mut self, rhs: &Self);
}
impl<T> RefAddAssign for T where T: Copy + AddAssign {
    fn ref_add_assign(&mut self, rhs: &Self) { *self += *rhs; }
}
pub trait RefSubAssign {
    fn ref_sub_assign(&mut self, rhs: &Self);
}
impl<T> RefSubAssign for T where T: Copy + SubAssign {
    fn ref_sub_assign(&mut self, rhs: &Self) { *self -= *rhs; }
}

/// Species how to calculate `a*b` given reference to `a` and `b`
pub trait RefMul {
    type Output;
    fn ref_mul(&self, rhs: &Self) -> Self::Output;
}
impl<T> RefMul for T where T: Copy + Mul<Output=T> {
    type Output = T;
    fn ref_mul(&self, rhs: &Self) -> Self::Output { *self * *rhs }
}

pub trait RefAdd {
    type Output;
    fn ref_add(&self, rhs: &Self) -> Self::Output;
}
impl<T> RefAdd for T where T: Copy + Add<Output=T> {
    type Output = T;
    fn ref_add(&self, rhs: &Self) -> Self::Output { *self + *rhs }
}
pub trait RefSub {
    type Output;
    fn ref_sub(&self, rhs: &Self) -> Self::Output;
}
impl<T> RefSub for T where T: Copy + Sub<Output=T> {
    type Output = T;
    fn ref_sub(&self, rhs: &Self) -> Self::Output { *self - *rhs }
}


pub trait LinearElem: Sized + Clone + AddZero + MulZero + RefAdd<Output=Self> + RefAddAssign 
    + RefMul<Output=Self> + RefMulAssign + RefSub<Output=Self> + RefSubAssign + RefInv
    + std::fmt::Display + std::fmt::Debug {}

/// Implements [`LinearElem`] for some primitives
mod implementations {
    use super::*;
    mod impl_f64 {
        use super::*;

        impl AddZero for f64 {
            fn add_zero() -> Self { 0.0 }
            fn is_add_zero(&self) -> bool { self.abs() <= 1e-6 }
        }
        impl MulZero for f64 {
            fn mul_zero() -> Self { 1.0 }
            fn is_mul_zero(&self) -> bool { (self - 1.0).abs() <= 1e-6 }
        }
        impl LinearElem for f64 {}
    }

    mod impl_f32 {
        use super::*;

        impl AddZero for f32 {
            fn add_zero() -> Self { 0.0 }
            fn is_add_zero(&self) -> bool { self.abs() <= 1e-6 }
        }
        impl MulZero for f32 {
            fn mul_zero() -> Self { 1.0 }
            fn is_mul_zero(&self) -> bool { (self - 1.0).abs() <= 1e-6 }
        }
        impl LinearElem for f32 {}

    }

    mod impl_i64 {
        use super::*;

        impl AddZero for i64 {
            fn add_zero() -> Self { 0 }
            fn is_add_zero(&self) -> bool { *self == 0 }
        }
        impl MulZero for i64 {
            fn mul_zero() -> Self { 1 }
            fn is_mul_zero(&self) -> bool { *self == 1 }
        }
        impl LinearElem for i64 {}

    }
    mod impl_i32 {
        use super::*;

        impl AddZero for i32 {
            fn add_zero() -> Self { 0 }
            fn is_add_zero(&self) -> bool { *self == 0 }
        }
        impl MulZero for i32 {
            fn mul_zero() -> Self { 1 }
            fn is_mul_zero(&self) -> bool { *self == 1 }
        }
        impl LinearElem for i32 {}

   }
}
