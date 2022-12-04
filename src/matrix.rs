use crate::element;
use crate::error::MatError;
use std::ops::{AddAssign, Mul, MulAssign, SubAssign};
use MatError::*;

pub mod alg;
mod concated_matrix;
mod data_matrix;
mod display;
mod eliminated_matrix;
mod mat_block;
mod slice_matrix;

#[cfg(test)]
mod test;

pub use concated_matrix::ConcatedMatrix;
pub use data_matrix::DataMatrix;
pub use display::{mat_print_buf, mat_to_string};
use element::*;
pub use eliminated_matrix::EliminatedMatrix;
pub use mat_block::MatBlock;
pub use slice_matrix::SliceMatrix;

/// The trait for a *matrix* that supports linear operations
pub trait Mat {
    type Item: LinearElem;

    /// Initialize some zeros
    ///
    /// All kinds of [`Mat`] need it, but only [`DataMatrix`] can be initialized with zeros
    /// as only it owns the data
    fn zeros(rows: usize, cols: usize) -> DataMatrix<Self::Item>
    where
        Self: Sized,
    {
        DataMatrix::zeros(rows, cols)
    }
    /// Initialize an identity
    ///
    /// For same reason as `zeros`, only [`DataMatrix`] can be used
    fn identity(rows: usize) -> DataMatrix<Self::Item>
    where
        Self: Sized,
    {
        DataMatrix::identity(rows)
    }

    /// Returns the number of rows in the *raw* form
    ///
    /// *Raw* means that rows here is determined by data layout in memory, regardless of transposed or
    /// not
    ///
    /// Should be implemented depending on data layout
    fn rows_raw(&self) -> usize;
    /// Return the number of cols in the *raw* form
    fn cols_raw(&self) -> usize;
    /// Return the number of rows, taken transpose into consideration
    fn rows(&self) -> usize {
        if self.is_transposed() {
            self.cols_raw()
        } else {
            self.rows_raw()
        }
    }
    /// Return the number of cols, taken transpose into consideration
    fn cols(&self) -> usize {
        if self.is_transposed() {
            self.rows_raw()
        } else {
            self.cols_raw()
        }
    }

    /// Returns `(rows(), cols())`
    fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }
    /// Returns if is square matrix
    fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }

    /// Indicates if the matrix is transposed
    ///
    /// Should be implemented depending on how transpose state is stored
    fn is_transposed(&self) -> bool;
    /// Transpose self
    ///
    /// Should be implemented depending on how transpose state is stored
    fn transpose(&mut self);
    /// Return self after transposing, for chain call
    fn transposed(mut self) -> Self
    where
        Self: Sized,
    {
        (&mut self).transpose();
        self
    }

    /// Get reference of the (i, j) element in the raw data layout,
    /// reagardless of transposed or not
    unsafe fn get_unchecked_raw(&self, i: usize, j: usize) -> &Self::Item;
    /// Get mutable reference of the (i, j) element in the raw data layout,
    /// reagardless of transposed or not
    unsafe fn get_mut_unchecked_raw(&mut self, i: usize, j: usize) -> &mut Self::Item;
    /// Transpose is taken into consideration, but index is not checked against the size of the matrix
    unsafe fn get_unchecked(&self, i: usize, j: usize) -> &Self::Item {
        if self.is_transposed() {
            self.get_unchecked_raw(j, i)
        } else {
            self.get_unchecked_raw(i, j)
        }
    }
    unsafe fn get_mut_unchecked(&mut self, i: usize, j: usize) -> &mut Self::Item {
        if self.is_transposed() {
            self.get_mut_unchecked_raw(j, i)
        } else {
            self.get_mut_unchecked_raw(i, j)
        }
    }
    /// Get reference of (i, j), checking if the index is in range
    fn get(&self, i: usize, j: usize) -> Result<&Self::Item, MatError> {
        if i >= self.rows() || j >= self.cols() {
            return Err(IndexError {
                dim: self.dimensions(),
                i,
                j,
                mutable: false,
            });
        }
        return unsafe { Ok(self.get_unchecked(i, j)) };
    }
    /// Get mutable reference of (i, j), checking index
    fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut Self::Item, MatError> {
        if i >= self.rows() || j >= self.cols() {
            return Err(IndexError {
                dim: self.dimensions(),
                i,
                j,
                mutable: true,
            });
        }
        return unsafe { Ok(self.get_mut_unchecked(i, j)) };
    }

    /// Scale the matrix
    fn scale(&mut self, s: &Self::Item) -> &mut Self
    where
        Self: Sized,
    {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                unsafe {
                    self.get_mut_unchecked(i, j).mul_assign(s);
                }
            }
        }
        self
    }

    /// Calculate `self` dot `rhs`, allocates a new [`DataMatrix`] holding the result
    ///
    /// Dimension is not checked
    unsafe fn dot_unchecked(&self, rhs: &dyn Mat<Item = Self::Item>) -> DataMatrix<Self::Item> {
        let mut result: DataMatrix<Self::Item> = DataMatrix::zeros(self.rows(), rhs.cols());

        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                for k in 0..self.cols() {
                    result.get_mut_unchecked(i, j).add_assign(
                        &self
                            .get_unchecked(i, k)
                            .clone()
                            .mul(rhs.get_unchecked(k, j)),
                    );
                }
            }
        }

        result
    }
    /// Check the dimension then call `dot_unchecked`
    fn dot(&self, rhs: &dyn Mat<Item = Self::Item>) -> Result<DataMatrix<Self::Item>, MatError> {
        if self.cols() != rhs.rows() {
            return Err(InconsistentDimension {
                need: (self.cols(), 0),
                got: rhs.dimensions(),
            });
        }
        return unsafe { Ok(self.dot_unchecked(rhs)) };
    }

    /// Add another matrix `rhs` to `self`
    ///
    /// If `rhs` is too large, it's truncated; If `rhs` is not large enough, rows and cols are repeated.
    /// More specificly, `(A + B)[ij] = A[ij] + B[i % b.rows(), j % b.cols()]`
    unsafe fn add_assign_unchecked(&mut self, rhs: &dyn Mat<Item = Self::Item>) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.get_mut_unchecked(i, j)
                    .add_assign(rhs.get_unchecked(i % rhs.rows(), j % rhs.cols()));
            }
        }
    }
    /// Same as `add_assign_unchecked`, but is marked safe
    fn add_assign(&mut self, rhs: &dyn Mat<Item = Self::Item>) {
        unsafe { self.add_assign_unchecked(rhs) }
    }
    /// Instead of assining the result to the original matrix, first clone a new one then add_assign it
    fn add(&self, rhs: &dyn Mat<Item = Self::Item>) -> DataMatrix<Self::Item> {
        let mut r = self.clone_data();
        r.add_assign(rhs);
        r
    }

    /// Same as `add_assign_unchecked`
    unsafe fn sub_assign_unchecked(&mut self, rhs: &dyn Mat<Item = Self::Item>) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.get_mut_unchecked(i, j)
                    .sub_assign(rhs.get_unchecked(i % rhs.rows(), j % rhs.cols()));
            }
        }
    }
    /// Same as `add_assign`
    fn sub_assign(&mut self, rhs: &dyn Mat<Item = Self::Item>) {
        unsafe { self.sub_assign_unchecked(rhs) }
    }
    /// Instead of assining the result to the original matrix, first clone a new one then sub_assign it
    fn sub(&self, rhs: &dyn Mat<Item = Self::Item>) -> DataMatrix<Self::Item> {
        let mut r = self.clone_data();
        r.sub_assign(rhs);
        r
    }

    /// Returns a [`SliceMatrix`] refering to the `i`th row of `self`
    ///
    /// The [`SliceMatrix`] is not affected by `transpose()`
    ///
    /// `i` is not checked against `self.rows()`
    unsafe fn row_unchecked(&self, i: usize) -> SliceMatrix<Self::Item>
    where
        Self: Sized,
    {
        slice_matrix::SliceMatrix::new_unchecked(self, i, 1, 0, self.cols())
    }
    /// Returns a [`SliceMatrix`] refering to the `j`th col of `self`
    ///
    /// `i` is not checked against `self.cols()`
    unsafe fn col_unchecked(&self, j: usize) -> SliceMatrix<Self::Item>
    where
        Self: Sized,
    {
        slice_matrix::SliceMatrix::new_unchecked(self, 0, self.rows(), j, 1)
    }

    /// Same as `col`
    fn row(&self, i: usize) -> Result<SliceMatrix<Self::Item>, MatError>
    where
        Self: Sized,
    {
        if i >= self.rows() {
            return Err(IndexError {
                dim: self.dimensions(),
                i,
                j: 0,
                mutable: true,
            });
        }
        return unsafe { Ok(self.row_unchecked(i)) };
    }
    /// Check if `j` is too big, then call `col_unchecked`
    fn col(&self, j: usize) -> Result<SliceMatrix<Self::Item>, MatError>
    where
        Self: Sized,
    {
        if j >= self.cols() {
            return Err(IndexError {
                dim: self.dimensions(),
                j,
                i: 0,
                mutable: true,
            });
        }
        return unsafe { Ok(self.row_unchecked(j)) };
    }

    /// Swap the data of `self` and `rhs`, dimension is not checked
    unsafe fn swap_unchecked(&mut self, rhs: &mut dyn Mat<Item = Self::Item>) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                std::mem::swap(self.get_mut_unchecked(i, j), rhs.get_mut_unchecked(i, j))
            }
        }
    }
    /// Check the dimension, then call `swap_unchecked`
    fn swap(&mut self, rhs: &mut dyn Mat<Item = Self::Item>) -> Result<(), MatError> {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            return Err(InconsistentDimension {
                need: self.dimensions(),
                got: rhs.dimensions(),
            });
        }
        return unsafe { Ok(self.swap_unchecked(rhs)) };
    }

    /// Checkout the elements within `self` and clone them to a newly-allocated [`DataMatrix`]
    fn clone_data(&self) -> DataMatrix<Self::Item> {
        let mut result = DataMatrix::zeros(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                unsafe { *result.get_mut_unchecked(i, j) = self.get_unchecked(i, j).clone() }
            }
        }
        result
    }

    /// Do the Gussian Elimination and get the [`EliminatedMatrix`] with pivot information
    /// for further use
    fn eliminated(self) -> EliminatedMatrix<Self::Item, Self>
    where
        Self: Sized,
        Self::Item: Inv,
    {
        EliminatedMatrix::eliminated(self)
    }
}

/// Implements how two trait object of [`Mat`] are equaled
impl<T> PartialEq<&dyn Mat<Item = T>> for &dyn Mat<Item = T>
where
    T: LinearElem,
{
    fn eq(&self, other: &&dyn Mat<Item = T>) -> bool {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return false;
        }

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                unsafe {
                    if !self.get_unchecked(i, j).eq(other.get_unchecked(i, j)) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
    fn ne(&self, other: &&dyn Mat<Item = T>) -> bool {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return true;
        }

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                unsafe {
                    if !self.get_unchecked(i, j).eq(other.get_unchecked(i, j)) {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}
