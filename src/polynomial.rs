use crate::element::*;

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

impl<T> RefAddAssign for Polynomial<T>
where
    T: LinearElem,
{
    fn ref_add_assign(&mut self, rhs: &Self) {
        if self.coef.len() < rhs.coef.len() {
            self.coef.resize(rhs.coef.len(), T::add_zero());
        }
        for (r, s) in rhs.coef.iter().zip(self.coef.iter_mut()) {
            s.ref_add_assign(r);
        }
    }
}

impl<T> RefAdd for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;
    fn ref_add(&self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();
        result.ref_add_assign(rhs);
        result
    }
}

impl<T> RefSubAssign for Polynomial<T>
where
    T: LinearElem,
{
    fn ref_sub_assign(&mut self, rhs: &Self) {
        if self.coef.len() < rhs.coef.len() {
            self.coef.resize(rhs.coef.len(), T::add_zero());
        }
        for (r, s) in rhs.coef.iter().zip(self.coef.iter_mut()) {
            s.ref_sub_assign(r);
        }
    }
}

impl<T> RefSub for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;
    fn ref_sub(&self, rhs: &Self) -> Self::Output {
        let mut result = self.clone();
        result.ref_sub_assign(rhs);
        result
    }
}

impl<T> RefMul for Polynomial<T>
where
    T: LinearElem,
{
    type Output = Self;

    fn ref_mul(&self, rhs: &Self) -> Self::Output {
        let mut result = Vec::new();
        result.resize(self.coef.len() + rhs.coef.len() - 1, T::add_zero());

        for (i, s) in self.coef.iter().enumerate() {
            for (j, r) in rhs.coef.iter().enumerate() {
                unsafe {
                    result
                        .get_unchecked_mut(i + j)
                        .ref_add_assign(&s.ref_mul(r));
                }
            }
        }

        Polynomial { coef: result }
    }
}

impl<T> RefMulAssign for Polynomial<T>
where
    T: LinearElem,
{
    fn ref_mul_assign(&mut self, rhs: &Self) {
        let mut result = self.ref_mul(rhs);
        std::mem::swap(&mut result, self);
    }
}

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
        assert_eq!(a().ref_add(&b()).coef, vec![2, 3, 1]);
        assert_eq!(b().ref_add(&a()).coef, vec![2, 3, 1]);
    }

    #[test]
    fn test_sub() {
        assert_eq!(a().ref_sub(&b()).coef, vec![0, 1, 1]);
        assert_eq!(b().ref_sub(&a()).coef, vec![0, -1, -1])
    }

    #[test]
    fn test_mul() {
        assert_eq!(a().ref_mul(&b()).coef, vec![1, 3, 3, 1]);
    }
}
