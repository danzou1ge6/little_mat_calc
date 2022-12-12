use crate::{Mat, element::*, DataMatrix, SliceMatrix};



pub fn orthogonalize<T: LinearElem + Inv>(vecs: &mut dyn Mat<Item = T>) {
    unsafe {
        for j in 1..vecs.cols() {
            let mut to_sub = DataMatrix::zeros(vecs.rows(), 1);
            let mut current_col = SliceMatrix::new_unchecked(vecs, 0, vecs.rows(), j, 1);

            for k in 0..j {
                let r = SliceMatrix::new_unchecked(vecs, 0, vecs.rows(), k, 1);
                let mut scale = current_col.dot_unchecked(&r).get_unchecked(0, 0).clone();
                let r_normal_sqr = r.dot_unchecked(&r).get_unchecked(0, 0).clone();
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
        let mut m: DataMatrix<Rational> = mat_![1 1; 0 1;].convert();
        orthogonalize(&mut m);
        assert_eq!(m.clone_data(), mat_![1 0; 0 1;].convert());
    }
}


