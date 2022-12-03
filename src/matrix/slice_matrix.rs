use std::marker::PhantomData;

use super::Mat;
use crate::element::LinearElem;
use crate::error::MatError;
use MatError::*;

/// A matrix that is a slice of another trait object of [`Mat`]
pub struct SliceMatrix<'a, T>
where
    T: LinearElem,
{
    pub origin: *mut dyn Mat<Item = T>,
    row_begin: usize,
    col_begin: usize,
    rows: usize,
    cols: usize,
    is_transposed: bool,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> SliceMatrix<'a, T>
where
    T: LinearElem,
{
    /// Create a new slice from any given matrix
    ///
    /// Caution: though `original` is behind a inmutable reference, it'll be converted to a
    /// `*mut` raw pointer, therefore it's actually mutable.
    /// This is done to allow `original` to be splitted into multiple slices
    pub unsafe fn new_unchecked(
        origin: &'a dyn Mat<Item = T>,
        row_begin: usize,
        rows: usize,
        col_begin: usize,
        cols: usize,
    ) -> Self {
        let origin = origin as *const dyn Mat<Item = T> as *mut dyn Mat<Item = T>;
        SliceMatrix {
            origin,
            row_begin,
            col_begin,
            rows,
            cols,
            is_transposed: false,
            _phantom: PhantomData,
        }
    }

    pub fn new(
        origin: &'a dyn Mat<Item = T>,
        row_begin: usize,
        rows: usize,
        col_begin: usize,
        cols: usize,
    ) -> Result<Self, MatError> {
        if row_begin + rows > origin.rows() || col_begin + cols > origin.cols() {
            return Err(IndexError {
                dim: origin.dimensions(),
                i: row_begin + rows,
                j: col_begin + cols,
                mutable: true,
            });
        }
        unsafe {
            Ok(SliceMatrix::new_unchecked(
                origin, row_begin, rows, col_begin, cols,
            ))
        }
    }
}

impl<'a, T> Mat for SliceMatrix<'a, T>
where
    T: LinearElem,
{
    type Item = T;

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
        (*self.origin).get_unchecked(i + self.row_begin, j + self.col_begin)
    }
    unsafe fn get_mut_unchecked_raw(&mut self, i: usize, j: usize) -> &mut T {
        (*self.origin).get_mut_unchecked(i + self.row_begin, j + self.col_begin)
    }
}
mod display {
    use super::super::mat_print_buf;
    use super::*;
    use std::fmt::{Debug, Display};

    impl<T> Display for SliceMatrix<'_, T>
    where
        T: LinearElem + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }

    impl<T> Debug for SliceMatrix<'_, T>
    where
        T: LinearElem + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mat_macro::mat_;

    #[test]
    fn test_slice_transpose() {
        let m = mat_![1 2; 3 4;];
        let slice = m.row(0).unwrap().transposed();

        assert_eq!(slice.clone_data(), mat_![1; 2;]);
    }
}
