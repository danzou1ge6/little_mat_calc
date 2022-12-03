use super::*;
use crate::element::*;

pub struct MatBlock<'a, T: LinearElem + 'a>(pub Box<dyn Mat<Item = T> + 'a>);

#[macro_export]
macro_rules! mat_block {
    ($x: expr) => {
        MatBlock(Box::new($x))
    };
}

mod display {

    use super::super::display::mat_print_buf;
    use super::*;
    use std::fmt::{Debug, Display};

    impl<'a, T> Display for MatBlock<'a, T>
    where
        T: LinearElem + 'a,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self.0.as_ref(), f)
        }
    }

    impl<'a, T> Debug for MatBlock<'a, T>
    where
        T: LinearElem + 'a,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self.0.as_ref(), f)
        }
    }
}

impl<'a, T> AddZero for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn add_zero() -> Self {
        MatBlock(Box::new(DataMatrix::zeros(1, 1)))
    }
    fn is_add_zero(&self) -> bool {
        for i in 0..self.0.rows() {
            for j in 0..self.0.cols() {
                unsafe {
                    if !self.0.get_unchecked(i, j).is_add_zero() {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

impl<'a, T> MulZero for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn is_mul_zero(&self) -> bool {
        for i in 0..self.0.rows() {
            for j in 0..self.0.cols() {
                unsafe {
                    if i == j && !self.0.get_unchecked(i, j).is_mul_zero() {
                        return false;
                    } else if i != j && !self.0.get_unchecked(i, j).is_add_zero() {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    fn mul_zero() -> Self {
        unimplemented!()
    }
}

impl<'a, T> RefSubAssign for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn ref_sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0.as_ref())
    }
}
impl<'a, T> RefAddAssign for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn ref_add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0.as_ref())
    }
}
impl<'a, T> RefMulAssign for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn ref_mul_assign(&mut self, rhs: &Self) {
        let result = self.0.dot(rhs.0.as_ref()).unwrap();
        let mut result = MatBlock(Box::new(result));
        std::mem::swap(self, &mut result);
    }
}

impl<'a, T> RefSub for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    type Output = Self;
    fn ref_sub(&self, rhs: &Self) -> Self::Output {
        let mut result = self.0.clone_data();
        result.sub_assign(rhs.0.as_ref());
        MatBlock(Box::new(result))
    }
}
impl<'a, T> RefAdd for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    type Output = Self;
    fn ref_add(&self, rhs: &Self) -> Self::Output {
        let mut result = self.0.clone_data();
        result.add_assign(rhs.0.as_ref());
        MatBlock(Box::new(result))
    }
}
impl<'a, T> RefMul for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    type Output = Self;
    fn ref_mul(&self, rhs: &Self) -> Self::Output {
        MatBlock(Box::new(self.0.dot(rhs.0.as_ref()).unwrap()))
    }
}

impl<'a, T> RefInv for MatBlock<'a, T>
where
    T: LinearElem + 'a + RefInv,
{
    fn inv(&self) -> Self {
        use crate::alg::inv;
        MatBlock(Box::new(inv(&mut self.0.clone_data()).unwrap()))
    }
}

impl<'a, T> Clone for MatBlock<'a, T>
where
    T: LinearElem + 'a,
{
    fn clone(&self) -> Self {
        MatBlock(Box::new(self.0.clone_data()))
    }
}

impl<'a, T> LinearElem for MatBlock<'a, T> where T: LinearElem + 'a {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::DataMatrix;
    use mat_macro::mat_;

    #[test]
    fn test_blocked_mat() {
        let a = mat_block!(mat_![1 2; 3 4;]);
        let b = mat_block!(mat_![1 0; 0 1;]);
        let c = mat_block!(mat_![0 0; 0 0;]);
        let d = mat_block!(mat_![2 3; 0 2;]);

        let m: DataMatrix<MatBlock<i32>> = mat_![
            a b;
            c d;
        ];
        // 1 2 1 0
        // 3 4 0 1
        // 0 0 2 3
        // 0 0 0 2

        m.row(0).unwrap().add_assign(
            m.row(1)
                .unwrap()
                .clone_data()
                .scale(&mat_block!(mat_![2 0; 0 2;])),
        );

        assert_eq!(
            m,
            mat_![
                (mat_block!(mat_![1 2; 3 4;])) (mat_block!(mat_![5 6; 0 5;]));
                (mat_block!(mat_![0 0; 0 0;])) (mat_block!(mat_![2 3; 0 2;]));
            ]
        );
    }
}
