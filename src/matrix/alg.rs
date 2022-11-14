use super::{Mat, MatError, DataMatrix, EliminatedMatrix};
use crate::element::LinearElem;
use MatError::*;

/// Caculate the inverted matrix, if any, of `mat`
/// 
/// If `mat` is not square, of course there is a [`MatError::DimensionError`];
/// And if the matrix is not invertable, returns a [`MatError::NotInvertable`] containing the rank;
/// 
/// Caution: this method ruins the original matrix, turning it into an identity
pub fn inv<T>(mat: &mut dyn Mat<Item=T>) -> Result<DataMatrix<T>, MatError> where T: LinearElem {

    use mat_macro::concated_mat_;

    if mat.rows() != mat.cols() {
        return Err(NotSquare { dim: mat.dimensions() });
    }

    let mut result = DataMatrix::identity(mat.rows());
    let augmented = concated_mat_![
        (mat) (&mut result);
    ].unwrap();

    let mut augmented = EliminatedMatrix::eliminated(augmented);
    augmented.reduce();

    if augmented.pivot_cols.last().unwrap().unwrap() >= augmented.rows() {
        let rank = augmented.pivot_cols
            .iter()
            .filter(
                |x|
                x.map_or(false, |x| x < augmented.rows())
            )
            .count();
        return Err(NotInvertable{rank, rows: augmented.rows()});
    }

    Ok(result)
}