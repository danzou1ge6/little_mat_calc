use super::Mat;
use crate::error::MatError;
use MatError::*;
use crate::element::LinearElem;

/// The matrix that is concated by many other matrix
pub struct ConcatedMatrix<'a, T> {
    data: Vec<&'a mut dyn Mat<Item=T>>,
    rows: usize,
    cols: usize,
    row_sizes: Vec<usize>,
    col_sizes: Vec<usize>,
    is_transposed: bool,
}

impl<'a, T> ConcatedMatrix<'a, T> where T: LinearElem {

    /// Calculate the index of the matrix block, and the index whtin the block holding
    /// the desired element
    unsafe fn blocked_index(&self, mut i: usize, mut j: usize) -> ((usize, usize), (usize, usize)) {
        let mut block_i = 0;
        while i >= *self.row_sizes.get_unchecked(block_i) {
            i -= self.row_sizes.get_unchecked(block_i);
            block_i += 1;
        }

        let mut block_j = 0;
        while j >= *self.col_sizes.get_unchecked(block_j) {
            j -= self.col_sizes.get_unchecked(block_j);
            block_j += 1;
        }

        ((block_i, block_j), (i, j))

    }

    /// Create a new [`ConcatedMatrix`] using vector of blocks and the dimension. The length of
    /// the blocks vector is checked against the dimension
    /// 
    /// If the dimension is knwon at compile time, consider using [`crate::concated_mat`] macro
    pub fn new(data: Vec<&'a mut dyn Mat<Item=T>>, rows: usize, cols: usize) -> Result<Self, MatError> {
        if data.len() != rows * cols {
            return Err(BadInitVector { len: data.len(), cols, rows });
        }

        let mut row_sizes: Vec<usize> = (0..rows).collect();
        for i in 0..rows { 
            row_sizes[i] = data[i * cols].rows();
            for j in 1..cols {
                if data[i * cols + j].rows() != row_sizes[i] {
                    return Err(ConcatFailure(
                        format!(
                            "Block at ({},{}) has rows={}, which is inconsistent with {}",
                            i, j, data[i * cols + j].rows(), row_sizes[i]
                        )
                    ));
                }
            }
        }

        let mut col_sizes: Vec<usize> = (0..cols).collect();
        for j in 0..cols {
            col_sizes[j] = data[j].cols();
            for i in 1..rows {
                if data[i * cols + j].cols() != col_sizes[j] {
                    return Err(ConcatFailure(
                        format!(
                            "Block at ({},{}) has cols={}, which is inconsistent with {}",
                            i, j, data[i * cols + j].cols(), col_sizes[j]
                        )
                    ));
                }
            }
        }

        Ok(ConcatedMatrix {
            data, rows: row_sizes.iter().sum(), cols: col_sizes.iter().sum(), row_sizes, col_sizes,
            is_transposed: false
        })
    }
}


impl<'a, T> Mat for ConcatedMatrix<'a, T> where T: LinearElem{
    type Item = T;

    fn transpose(&mut self) { self.is_transposed = !self.is_transposed; }
    fn is_transposed(&self) -> bool { self.is_transposed }
    fn rows_raw(&self) -> usize { self.rows }
    fn cols_raw(&self) -> usize { self.cols }

    unsafe fn get_unchecked_raw(&self, i: usize, j: usize) -> &T {
        let ((block_i, block_j), (i, j)) = self.blocked_index(i, j);
        self.data.get_unchecked(block_i * self.col_sizes.len() + block_j).get_unchecked(i, j)
    }

    unsafe fn get_mut_unchecked_raw(&mut self, i: usize, j: usize) -> &mut T {
        let ((block_i, block_j), (i, j)) = self.blocked_index(i, j);
        self.data.get_unchecked_mut(block_i * self.col_sizes.len() + block_j).get_mut_unchecked(i, j)
    }


}

mod display {
    use std::fmt::{Debug, Display};
    use super::*;
    use super::super::mat_print_buf;
    

    impl<T> Display for ConcatedMatrix<'_, T> where T: LinearElem + Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { mat_print_buf(self, f) }
    }

    impl<T> Debug for ConcatedMatrix<'_, T> where T: LinearElem + Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { mat_print_buf(self, f) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::DataMatrix;
    use mat_macro::{mat_, concated_mat_};
    
    #[test]
    fn test_concat_transpose() {
        let mut a: DataMatrix<i32> = mat_![1 2;];
        let mut b: DataMatrix<i32> = mat_![3 4;];
        
        let m = concated_mat_![(&mut a); (&mut b);].unwrap().transposed();

        assert_eq!(m.clone_data(), mat_![1 3; 2 4;]);
    }
}

