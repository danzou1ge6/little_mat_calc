mod inv_mat {
    use super::super::{DataMatrix, EliminatedMatrix, Mat, MatError};
    use crate::element::{LinearElem, Inv};
    use MatError::*;

    /// Caculate the inverted matrix, if any, of `mat`
    ///
    /// If `mat` is not square, of course there is a [`MatError::DimensionError`];
    /// And if the matrix is not invertable, returns a [`MatError::NotInvertable`] containing the rank;
    ///
    /// Caution: this method ruins the original matrix, turning it into an identity
    pub fn inv<T>(mat: &mut dyn Mat<Item = T>) -> Result<DataMatrix<T>, MatError>
    where
        T: LinearElem + Inv,
    {
        use mat_macro::concated_mat_;

        if mat.rows() != mat.cols() {
            return Err(NotSquare {
                dim: mat.dimensions(),
            });
        }

        let mut result = DataMatrix::identity(mat.rows());
        let augmented = concated_mat_![
            (mat) (&mut result);
        ]
        .unwrap();

        let mut augmented = EliminatedMatrix::eliminated(augmented);
        augmented.reduce();

        if augmented.pivot_cols.last().unwrap().unwrap() >= augmented.rows() {
            let rank = augmented
                .pivot_cols
                .iter()
                .filter(|x| x.map_or(false, |x| x < augmented.rows()))
                .count();
            return Err(NotInvertable {
                rank,
                rows: augmented.rows(),
            });
        }

        Ok(result)
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_inv() {
            use crate::alg;
            use crate::DataMatrix;
            use crate::Rational;
            use mat_macro::mat_;

            let mut a: DataMatrix<Rational> = mat_![
                1 2;
                3 1;
            ]
            .convert();
            let inv = alg::inv(&mut a).unwrap();

            assert_eq!(
                inv,
                mat_![
                    (Rational(-1, 5)) (Rational(2, 5));
                    (Rational(3, 5)) (Rational(-1, 5));
                ]
            );
        }
    }
}
pub use inv_mat::inv;

mod det_mat;
pub use det_mat::det;

mod trace {
    use crate::element::LinearElem;
    use crate::error::MatError;
    use crate::Mat;

    pub fn trace<T>(mat: &dyn Mat<Item = T>) -> Result<T, MatError>
    where
        T: LinearElem,
    {
        if mat.rows() != mat.cols() {
            return Err(MatError::NotSquare {
                dim: mat.dimensions(),
            });
        } else {
            let mut tr = T::add_zero();
            for i in 0..mat.rows() {
                unsafe {
                    tr.add_assign(mat.get_unchecked(i, i));
                }
            }
            return Ok(tr);
        }
    }
}
pub use trace::trace;

mod linear_equation;
pub use linear_equation::{solve, solve_augmented, SolveResult};
