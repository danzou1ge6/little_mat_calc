use crate::matrix::Mat;
use crate::error::MatError;
use MatError::*;

/// Calculates the square of normal of the `j`th row of `mat`
pub unsafe fn col_normal_sqr_unchecked(mat: &dyn Mat<Item = f64>, j: usize) -> f64 {
    let mut sqr_sum = 0.0;

    for i in 0..mat.rows() {
        sqr_sum += mat.get_unchecked(i, j).powi(2);
    }

    sqr_sum
}

/// Calculates the normal of the `j`th row of `mat`
pub unsafe fn col_normal_unchecked(mat: &dyn Mat<Item = f64>, j: usize) -> f64 {
    col_normal_sqr_unchecked(mat, j).sqrt()
}

pub fn col_normal(mat: &dyn Mat<Item = f64>, j: usize) -> Result<f64, MatError> {
    if j >= mat.cols() {
        return Err(IndexError { dim: mat.dimensions(), i: 0, j, mutable: false });
    }

    unsafe { Ok(col_normal_unchecked(mat, j)) }
}

pub fn col_normal_sqr(mat: &dyn Mat<Item = f64>, j: usize) -> Result<f64, MatError> {
    if j >= mat.cols() {
        return Err(IndexError { dim: mat.dimensions(), i: 0, j, mutable: false });
    }

    unsafe { Ok(col_normal_sqr_unchecked(mat, j)) }
}

/// Calculates the square of normal of the `i`th col of `mat`
pub unsafe fn row_normal_sqr_unchecked(mat: &dyn Mat<Item = f64>, i: usize) -> f64 {
    let mut sqr_sum = 0.0;

    for j in 0..mat.cols() {
        sqr_sum += mat.get_unchecked(i, j).powi(2);
    }

    sqr_sum
}

/// Calculates the normal of the `j`th row of `mat`
pub unsafe fn row_normal_unchecked(mat: &dyn Mat<Item = f64>, i: usize) -> f64 {
    row_normal_sqr_unchecked(mat, i).sqrt()
}

pub fn row_normal(mat: &dyn Mat<Item = f64>, i: usize) -> Result<f64, MatError> {
    if i >= mat.cols() {
        return Err(IndexError { dim: mat.dimensions(), i, j: 0, mutable: false });
    }

    unsafe { Ok(row_normal_unchecked(mat, i)) }
}

pub fn row_normal_sqr(mat: &dyn Mat<Item = f64>, i: usize) -> Result<f64, MatError> {
    if i >= mat.cols() {
        return Err(IndexError { dim: mat.dimensions(), i, j: 0, mutable: false });
    }

    unsafe { Ok(row_normal_sqr_unchecked(mat, i)) }
}


#[cfg(test)]
mod test {
    use super::*;
    use mat_macro::mat_;

    #[test]
    fn test_row_normal() {
        let v = mat_![1.0 2.0 3.0;];
        let n = (row_normal_sqr(&v, 0).unwrap() - 14.0).abs();
        assert!(n < 1e-3);
    }
    #[test]
    fn test_col_normal() {
        let v = mat_![1.0; 2.0; 3.0;];
        let n = (col_normal_sqr(&v, 0).unwrap() - 14.0).abs();
        assert!(n < 1e-3);
    }

}