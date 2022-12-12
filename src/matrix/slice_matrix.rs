use std::cell::{Ref, RefCell};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use super::Mat;
use crate::element::LinearElem;
use crate::error::MatError;
use MatError::*;

/// A matrix that is a slice of another trait object of [`Mat`]
/// Caution: This struct may lead to data race, don't use in multi-threading!
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
    _phantom: PhantomData<&'a i32>,
}

impl<T> !Sync for SliceMatrix<'_, T> {}

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

impl<'a, T> Clone for SliceMatrix<'a, T>
where
    T: LinearElem,
{
    fn clone(&self) -> Self {
        Self {
            origin: self.origin,
            row_begin: self.row_begin,
            col_begin: self.col_begin,
            rows: self.rows,
            cols: self.cols,
            is_transposed: self.is_transposed,
            _phantom: PhantomData,
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

/// A struct that holds the [`Ref`] to the [`Rc<RefCell>`], to indicate that the origin
/// is borrowed in the lifetime of [`SliceRef`]
pub struct SliceRef<'b, T>
where
    T: LinearElem,
{
    borrow: Ref<'b, dyn Mat<Item = T>>,
    slice: &'b mut SliceMatrix<'static, T>,
}

/// A slice matrix, unlike [`SliceMatrix`], this also holds the [`Rc<RefCell>`] of
/// the origin matrix. Also like [`SliceMatrix`], one origin may be mutably borrowed
/// at many time, so data race may happen, therefore never use this in multi-threading
pub struct RcSliceMatrix<T>
where
    T: LinearElem,
{
    origin: Rc<RefCell<dyn Mat<Item = T>>>,
    slice: SliceMatrix<'static, T>,
}

impl<T> RcSliceMatrix<T>
where
    T: LinearElem + 'static,
{
    pub fn new(
        origin: Rc<RefCell<dyn Mat<Item = T>>>,
        row_begin: usize,
        rows: usize,
        col_begin: usize,
        cols: usize,
    ) -> Result<Self, MatError> {
        let p = origin.as_ptr();
        let p = unsafe { &(*p) };

        let slice = SliceMatrix::new(p, row_begin, rows, col_begin, cols)?;

        Ok(RcSliceMatrix { origin, slice })
    }

    pub fn borrow(&mut self) -> SliceRef<'_, T> {
        let borrow = self.origin.as_ref().borrow();

        SliceRef {
            borrow,
            slice: &mut self.slice,
        }
    }
}

impl<'b, T> Deref for SliceRef<'b, T>
where
    T: LinearElem,
{
    type Target = SliceMatrix<'static, T>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}

impl<'b, T> DerefMut for SliceRef<'b, T>
where
    T: LinearElem,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (*self.slice)
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

    #[test]
    fn test_rc() {
        let m = mat_![1 2; 3 4;];
        let m = Rc::new(RefCell::new(m));
        let mut slice = RcSliceMatrix::new(m.clone(), 0, 2, 0, 1).unwrap();
        let mut slice2 = RcSliceMatrix::new(m.clone(), 0, 1, 0, 2).unwrap();
        let bm = slice.borrow();
        let mut bm2 = slice2.borrow();

        assert_eq!(bm.deref().clone_data(), mat_![1; 3;]);
        assert_eq!(bm2.deref().clone_data(), mat_![1 2;]);

        *bm2.get_mut(0, 0).unwrap() = 0;

        assert_eq!(m.borrow().clone_data(), mat_![0 2; 3 4;]);

        assert_eq!(Rc::strong_count(&m), 3);

        drop(bm);
        drop(slice);
        assert_eq!(Rc::strong_count(&m), 2);

        drop(bm2);
        drop(slice2);
        assert_eq!(Rc::strong_count(&m), 1);
    }
}
