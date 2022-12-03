use super::Mat;
use crate::element::{LinearElem, RefEq};
use crate::error::MatError;
use MatError::*;

/// The fundamental matrix that holds the data
pub struct DataMatrix<T> {
    data: Vec<T>,
    cols: usize,
    rows: usize,
    is_transposed: bool,
}

impl<T> DataMatrix<T>
where
    T: LinearElem,
{
    /// Create a new [`DataMatrix`] using its elements and dimensions
    ///
    /// `data` is concated rows of the matrix. Its length is checked against the dimension
    ///
    /// To create a [`DataMatrix`] with static dimension, [`crate::mat`] macro may come handy
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Result<Self, MatError> {
        if data.len() != cols * rows {
            return Err(BadInitVector {
                len: data.len(),
                cols,
                rows,
            });
        }
        Ok(DataMatrix {
            data,
            rows,
            cols,
            is_transposed: false,
        })
    }

    pub unsafe fn new_unchecked(data: Vec<T>, rows: usize, cols: usize) -> Self {
        DataMatrix {
            data,
            cols,
            rows,
            is_transposed: false,
        }
    }

    /// Explicitly convert a [`DataMatrix`]'s data to another type
    pub fn convert<U: From<T>>(self) -> DataMatrix<U> {
        DataMatrix {
            data: self.data.into_iter().map(|x| x.into()).collect(),
            rows: self.rows,
            cols: self.cols,
            is_transposed: false,
        }
    }
}

impl<T> Mat for DataMatrix<T>
where
    T: LinearElem,
{
    type Item = T;

    /// Returns a [`DataMatrix`] holding all zeros
    fn zeros(rows: usize, cols: usize) -> Self {
        let mut data = Vec::new();
        data.resize(rows * cols, T::add_zero());
        DataMatrix {
            data,
            cols,
            rows,
            is_transposed: false,
        }
    }
    /// Returns a [`DataMatrix`] which is an identity
    fn identity(rows: usize) -> Self {
        let mut matrix = Self::zeros(rows, rows);
        for i in 0..rows {
            unsafe {
                *matrix.get_mut_unchecked(i, i) = T::mul_zero();
            }
        }
        matrix
    }

    fn transpose(&mut self) {
        self.is_transposed = !self.is_transposed;
    }
    fn is_transposed(&self) -> bool {
        self.is_transposed
    }
    fn rows_raw(&self) -> usize {
        self.rows
    }
    fn cols_raw(&self) -> usize {
        self.cols
    }

    unsafe fn get_unchecked_raw(&self, i: usize, j: usize) -> &T {
        self.data.get_unchecked(i * self.cols + j)
    }
    unsafe fn get_mut_unchecked_raw(&mut self, i: usize, j: usize) -> &mut T {
        self.data.get_unchecked_mut(i * self.cols + j)
    }
}

impl<T: LinearElem> PartialEq for DataMatrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return false;
        }

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                unsafe {
                    if !self.get_unchecked(i, j).ref_eq(other.get_unchecked(i, j)) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}
impl<T: LinearElem> Eq for DataMatrix<T> {}

mod display {
    use super::super::mat_print_buf;
    use super::*;
    use std::fmt::{Debug, Display};

    impl<T> Display for DataMatrix<T>
    where
        T: LinearElem + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }

    impl<T> Debug for DataMatrix<T>
    where
        T: LinearElem + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }
}
