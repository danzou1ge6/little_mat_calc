use crate::element::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::cmp::{PartialEq, Eq};

#[macro_export]
/// Create a [`Polynomial`], coefficients are from low to high
macro_rules! polynomial {
    ($($x:expr),+ $(,)?) => {
        $crate::Polynomial { coef: vec![$($x),+] }
    };
}

/// Represents a polynomial
#[derive(Clone)]
pub struct Polynomial<T>
where
    T: LinearElem,
{
    /// Coefficents are stored from low to high, for example,
    /// `x^2 + 2x + 3` is stored in `vec![3, 2, 1]`
    pub coef: Vec<T>,
}

mod display {
    use super::*;
    use std::fmt::{Debug, Display};

    impl<T> Display for Polynomial<T>
    where
        T: LinearElem,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, a) in self.coef.iter().enumerate() {
                if i == 0 {
                    write!(f, "{}", a)?;
                } else if i == 1 {
                    write!(f, " + {} x", a)?;
                } else {
                    write!(f, " + {} x^{}", a, i)?;
                }
            }
            write!(f, "")
        }
    }
    impl<T> Debug for Polynomial<T>
    where
        T: LinearElem,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (self as &dyn Display).fmt(f)
        }
    }
}

impl<T> AddZero for Polynomial<T>
where
    T: LinearElem,
{
    fn add_zero() -> Self {
        Polynomial {
            coef: vec![T::add_zero()],
        }
    }
    fn is_add_zero(&self) -> bool {
        self.coef.len() == 1 && self.coef.first().unwrap().is_add_zero()
    }
}

impl<T> MulZero for Polynomial<T>
where
    T: LinearElem,
{
    fn mul_zero() -> Self {
        Polynomial {
            coef: vec![T::mul_zero()],
        }
    }
    fn is_mul_zero(&self) -> bool {
        self.coef.len() == 1 && self.coef.first().unwrap().is_mul_zero()
    }
}

impl<T> AddAssign<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    fn add_assign(&mut self, rhs: &Self) {
        if self.coef.len() < rhs.coef.len() {
            self.coef.resize(rhs.coef.len(), T::add_zero());
        }
        for (r, s) in rhs.coef.iter().zip(self.coef.iter_mut()) {
            s.add_assign(r);
        }
    }
}

impl<T> Add<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();
        result.add_assign(rhs);
        result
    }
}

impl<T> SubAssign<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    fn sub_assign(&mut self, rhs: &Self) {
        if self.coef.len() < rhs.coef.len() {
            self.coef.resize(rhs.coef.len(), T::add_zero());
        }
        for (r, s) in rhs.coef.iter().zip(self.coef.iter_mut()) {
            s.sub_assign(r);
        }
    }
}

impl<T> Sub<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();
        result.sub_assign(rhs);
        result
    }
}

impl<T> Mul<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut result = Vec::new();
        result.resize(self.coef.len() + rhs.coef.len() - 1, T::add_zero());

        for (i, s) in self.coef.iter().enumerate() {
            for (j, r) in rhs.coef.iter().enumerate() {
                unsafe {
                    result
                        .get_unchecked_mut(i + j)
                        .add_assign(&s.clone().mul(r));
                }
            }
        }

        Polynomial { coef: result }
    }
}

impl<T> MulAssign<&Self> for Polynomial<T>
where
    T: LinearElem,
{
    fn mul_assign(&mut self, rhs: &Self) {
        unsafe {
            let taken_self = std::ptr::read(self);
            let result = taken_self.mul(rhs);
            std::ptr::write(self, result)
        }
    }
}

impl<T> PartialEq for Polynomial<T> where T: PartialEq + LinearElem {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T> Eq for Polynomial<T> where T: Eq + LinearElem {}

impl<T> LinearElem for Polynomial<T> where T: LinearElem {}

impl<T> From<T> for Polynomial<T>
where
    T: LinearElem,
{
    fn from(x: T) -> Self {
        Polynomial { coef: vec![x] }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn a() -> Polynomial<i32> {
        polynomial!(1, 2, 1)
    }
    fn b() -> Polynomial<i32> {
        Polynomial { coef: vec![1, 1] }
    }

    #[test]
    fn test_add() {
        assert_eq!(a().add(&b()).coef, vec![2, 3, 1]);
        assert_eq!(b().add(&a()).coef, vec![2, 3, 1]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(a().sub(&b()).coef, vec![0, 1, 1]);
        assert_eq!(b().sub(&a()).coef, vec![0, -1, -1])
    }

    #[test]
    fn test_mul() {
        assert_eq!(a().mul(&b()).coef, vec![1, 3, 3, 1]);
    }
    
    #[test]
    fn test_mul_assign() {
        let mut a = a();
        a.mul_assign(&b());
        assert_eq!(a.coef, vec![1, 3, 3, 1]);
    }
}
