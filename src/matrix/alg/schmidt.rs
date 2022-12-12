use crate::{Mat, element::*, DataMatrix, SliceMatrix};


/// Apply schmidt procedure on columns of `vecs`, but not normalizing
pub fn orthogonalize<T: LinearElem + Inv>(vecs: &mut dyn Mat<Item = T>) {
    unsafe {
        for j in 1..vecs.cols() {
            let mut to_sub = DataMatrix::zeros(vecs.rows(), 1);
            let mut current_col = SliceMatrix::col_unchecked(vecs, j);

            for k in 0..j {
                let r = SliceMatrix::col_unchecked(vecs, k);
                let mut r2 = r.clone();
                r2.transpose();

                let mut scale = r2.dot_unchecked(&current_col).get_unchecked(0, 0).clone();
                let r_normal_sqr = r2.dot_unchecked(&r).get_unchecked(0, 0).clone();
                scale.mul_assign(&r_normal_sqr.inv());

                to_sub.add_assign_unchecked(r.clone_data().scale(&scale));
            }

            current_col.sub_assign_unchecked(&to_sub);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use mat_macro::mat_;
    use crate::*;

    #[test]
    fn test_orthogonalize() {
        let mut m: DataMatrix<Rational> = mat_![2 -1 14; -1 5 1; -3 1 9;].convert();
        orthogonalize(&mut m);
        assert_eq!(m.clone_data(), mat_![
            (Rational(2, 1)) (Rational(3, 7)) (Rational(14, 1));
            (Rational(-1, 1)) (Rational(30, 7)) (Rational(1, 1));
            (Rational(-3, 1)) (Rational(-8, 7)) (Rational(9, 1));
        ].convert());
    }
}


