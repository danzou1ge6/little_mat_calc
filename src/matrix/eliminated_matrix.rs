use super::DataMatrix;
use super::Mat;
use super::SliceMatrix;
use crate::element::*;

/// Implementation of Gussian Elimination
pub unsafe fn elimination<T, M: Mat<Item = T>>(mat: &mut M) -> [Vec<Option<usize>>; 2]
where
    T: LinearElem + RefInv,
{
    let mut pivot_col = 0;
    let mut pivot_row = 0;

    let mut pivot_cols: Vec<Option<usize>> = (0..mat.rows()).into_iter().map(|_| None).collect();
    let mut pivot_rows: Vec<Option<usize>> = (0..mat.cols()).into_iter().map(|_| None).collect();

    while pivot_col < mat.cols() && pivot_row < mat.rows() {
        // look for the first non_zero element in `pivot_col` and swap it to the first row
        let mut found_non_zero = false;
        for i in pivot_row..mat.rows() {
            if !mat.get_unchecked(i, pivot_col).is_add_zero() {
                if i != pivot_row {
                    mat.row_unchecked(pivot_row)
                        .swap_unchecked(&mut mat.row_unchecked(i));
                }
                found_non_zero = true;
                break;
            }
        }

        // eliminate the zeros in following rows
        if found_non_zero {
            for i in pivot_row + 1..mat.rows() {
                // if the row is already zero
                if mat.get_unchecked(i, pivot_col).is_add_zero() {
                    continue;
                }

                mat.row_unchecked(i).sub_assign_unchecked(
                    mat.row_unchecked(pivot_row).clone_data().scale(
                        &mat.get_unchecked(i, pivot_col)
                            .ref_mul(&mat.get_unchecked(pivot_row, pivot_col).inv()),
                    ),
                );
            }
            *pivot_cols.get_unchecked_mut(pivot_row) = Some(pivot_col);
            *pivot_rows.get_unchecked_mut(pivot_col) = Some(pivot_row);
            pivot_row += 1;
        }

        // proceed to next pivot_col
        pivot_col += 1;
    }

    [pivot_rows, pivot_cols]
}

/// An eliminated matrix holidng information of where pivots are
pub struct EliminatedMatrix<T, M>
where
    M: Mat<Item = T>,
    T: LinearElem + RefInv,
{
    /// The data
    pub mat: M,
    /// `pivot_cols[row] = Some(col)` where (row, col) is a pivot;
    /// if there is no pivot in `row`, `pivot_cols[row] = None`
    pub pivot_cols: Vec<Option<usize>>,
    /// Same as `pivot_cols`, but the other way around
    pub pivot_rows: Vec<Option<usize>>,
}

impl<T, M> EliminatedMatrix<T, M>
where
    M: Mat<Item = T>,
    T: LinearElem + RefInv,
{
    /// Eliminate a matrix and stores it in a [`EliminatedMatrix`]
    pub fn eliminated(mut mat: M) -> Self {
        let [pivot_rows, pivot_cols] = unsafe { elimination(&mut mat) };
        Self {
            mat,
            pivot_cols,
            pivot_rows,
        }
    }

    /// Scale each row so that pivots are `1`s
    pub fn simplify(&mut self) {
        unsafe {
            for i in 0..self.rows() {
                match self.pivot_cols.get_unchecked(i) {
                    None => break,
                    Some(pivot_col) => {
                        self.row_unchecked(i)
                            .scale(&self.get_unchecked(i, *pivot_col).inv());
                    }
                }
            }
        }
    }
    pub fn simplified(mut self) -> Self {
        self.simplify();
        self
    }

    /// Using row transformations to make all elements above a pivot zeros
    pub fn reduce(&mut self) {
        self.simplify();

        unsafe {
            for i in 0..self.rows() {
                // check each row for pivot
                match self.pivot_cols.get_unchecked(i) {
                    // if one row has no pivot, then rows after neither
                    None => break,
                    Some(pivot_col) => {
                        // check for pivots right-bottom of current one
                        for j in pivot_col + 1..self.cols() {
                            match self.pivot_rows.get_unchecked(j) {
                                // some cols may have no pivots
                                None => continue,
                                Some(pivot_row) => {
                                    // subtract the found pivot from current row
                                    self.row_unchecked(i).sub_assign_unchecked(
                                        self.row_unchecked(*pivot_row)
                                            .clone_data()
                                            .scale(self.get_unchecked(i, j)),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn reduced(mut self) -> Self {
        self.reduce();
        self
    }

    /// Count the rank of the matrix
    pub fn rank(&self) -> usize {
        let mut r = 0;
        for item in self.pivot_cols.iter() {
            if item.is_some() {
                r += 1
            } else {
                break;
            }
        }
        r
    }

    /// Return a [`DataMatrix`] cols of which holds the basis for the null space, if null space is
    /// not {0};
    /// Otherwise returns [`None`]
    pub fn null_space(&self) -> Option<DataMatrix<T>> {
        if self.rank() == self.cols() {
            return None;
        };

        let result = DataMatrix::zeros(self.cols(), self.cols() - self.rank());

        unsafe {
            let free_vars: Vec<usize> = (0..self.cols())
                .filter(|i| self.pivot_rows.get_unchecked(*i).is_none())
                .collect();
            let pivot_vars: Vec<usize> = (0..self.cols())
                .filter(|i| self.pivot_rows.get_unchecked(*i).is_some())
                .collect();

            for (i, free) in free_vars.iter().rev().enumerate() {
                let mut sol = DataMatrix::zeros(self.cols(), 1);
                *sol.get_mut_unchecked(*free, 0) = T::mul_zero();

                for pivot in pivot_vars.iter().rev() {
                    let sol_slice =
                        SliceMatrix::new_unchecked(&sol, pivot + 1, self.cols() - pivot, 0, 1);
                    let coef_slice = SliceMatrix::new_unchecked(
                        self,
                        self.pivot_rows.get_unchecked(*pivot).unwrap(),
                        1,
                        pivot + 1,
                        self.cols() - pivot,
                    );

                    let neg_pivot_val = coef_slice.dot_unchecked(&sol_slice);
                    let neg_pivot_val = neg_pivot_val.get_unchecked(0, 0);
                    *sol.get_mut_unchecked(*pivot, 0) = T::add_zero().ref_sub(neg_pivot_val);
                }

                result.col_unchecked(i).add_assign_unchecked(&sol);
            }
        }

        Some(result)
    }

    /// Calculate the special solution of `self*x=b`, where all free variables are set to `0`
    ///
    /// If their is no solution, returns [`None`]
    pub fn special_solution(&self, b: &dyn Mat<Item = T>) -> Option<DataMatrix<T>> {
        let mut sol = DataMatrix::zeros(self.cols(), 1);

        unsafe {
            for i in (0..self.rows()).into_iter().rev() {
                match self.pivot_cols.get_unchecked(i) {
                    None => {
                        if !b.get_unchecked(i, 0).is_add_zero() {
                            return None;
                        }
                    }
                    Some(pivot) => {
                        let sol_slice =
                            SliceMatrix::new_unchecked(&sol, pivot + 1, self.cols() - pivot, 0, 1);
                        let coef_slice =
                            SliceMatrix::new_unchecked(self, i, 1, pivot + 1, self.cols() - pivot);

                        let tmp = coef_slice.dot_unchecked(&sol_slice);
                        let tmp = tmp.get_unchecked(0, 0);
                        *sol.get_mut_unchecked(*pivot, 0) = b.get_unchecked(i, 0).ref_sub(tmp)
                    }
                }
            }
        }

        Some(sol)
    }

    /// Returns a slice of `self`, retaining the pivot information
    ///
    /// Caution: `self` is actually mutably borrowed, for details refer to `new` method of
    /// [`SliceMatrix`]
    pub fn slice(
        &self,
        row_begin: usize,
        rows: usize,
        col_begin: usize,
        cols: usize,
    ) -> EliminatedMatrix<T, SliceMatrix<T>> {
        let slice_mat = SliceMatrix::new(self, row_begin, rows, col_begin, cols).unwrap();
        let pivot_cols = self.pivot_cols[row_begin..row_begin + rows]
            .iter()
            .map(|&x| {
                x.map_or(None, |x| {
                    if col_begin <= x && x < col_begin + cols {
                        Some(x)
                    } else {
                        None
                    }
                })
            })
            .collect();
        let pivot_rows = self.pivot_rows[col_begin..col_begin + cols]
            .iter()
            .map(|x| {
                x.map_or(None, |x| {
                    if row_begin <= x && x < row_begin + rows {
                        Some(x)
                    } else {
                        None
                    }
                })
            })
            .collect();

        EliminatedMatrix {
            mat: slice_mat,
            pivot_cols,
            pivot_rows,
        }
    }
}

impl<T, M> Mat for EliminatedMatrix<T, M>
where
    M: Mat<Item = T>,
    T: LinearElem + RefInv,
{
    type Item = T;

    fn is_transposed(&self) -> bool {
        self.mat.is_transposed()
    }
    fn transpose(&mut self) {
        self.mat.transpose();
        std::mem::swap(&mut self.pivot_cols, &mut self.pivot_rows);
    }
    fn rows_raw(&self) -> usize {
        self.mat.rows_raw()
    }
    fn cols_raw(&self) -> usize {
        self.mat.cols_raw()
    }

    unsafe fn get_unchecked_raw(&self, i: usize, j: usize) -> &Self::Item {
        self.mat.get_unchecked_raw(i, j)
    }
    unsafe fn get_mut_unchecked_raw(&mut self, i: usize, j: usize) -> &mut Self::Item {
        self.mat.get_mut_unchecked_raw(i, j)
    }
}

mod display {
    use super::super::mat_print_buf;
    use super::*;
    use std::fmt::{Debug, Display};

    impl<T, M> Display for EliminatedMatrix<T, M>
    where
        M: Mat<Item = T>,
        T: LinearElem + Display + RefInv,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }

    impl<T, M> Debug for EliminatedMatrix<T, M>
    where
        M: Mat<Item = T>,
        T: LinearElem + Display + RefInv,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            mat_print_buf(self, f)
        }
    }
}

#[cfg(test)]
mod test {

    use super::super::{DataMatrix, EliminatedMatrix, Mat};
    use crate::Rational;
    use mat_macro::mat_;

    fn eliminate() -> EliminatedMatrix<Rational, DataMatrix<Rational>> {
        let a: DataMatrix<i32> = mat_![
            2   1  -1   1   1;
            3  -2   1  -3   4;
            1   4  -3   5  -2;
        ];
        let b: DataMatrix<Rational> = a.convert();

        let b = EliminatedMatrix::eliminated(b);

        assert_eq!(&b as &dyn Mat<Item=_>, &mat_![
            (Rational(2, 1))  (Rational(1, 1))  (Rational(-1, 1))  (Rational(1, 1)) (Rational(1, 1));
            (Rational(0, 1))  (Rational(-7, 2)) (Rational(5, 2))  (Rational(-9, 2))  (Rational(5, 2));
            (Rational(0, 1))  (Rational(0, 1))   (Rational(0, 1))  (Rational(0, 1)) (Rational(0, 1));
        ].convert() as &dyn Mat<Item=_>);

        b
    }

    #[test]
    fn test_elimination() {
        eliminate();
    }

    #[test]
    fn test_transpose_elimination() {
        let a: DataMatrix<Rational> = mat_![
            2   3    1;
            1  -2    4;
           -1   1   -3;
            1  -3    5;
            1   4   -2;
        ]
        .convert();

        let b = a.transposed().eliminated();

        assert_eq!(
            &b as &dyn Mat<Item = _>,
            &mat_![
                (Rational(2, 1))  (Rational(1, 1))  (Rational(-1, 1))  (Rational(1, 1)) (Rational(1, 1));
                (Rational(0, 1))  (Rational(-7, 2)) (Rational(5, 2))  (Rational(-9, 2))  (Rational(5, 2));
                (Rational(0, 1))  (Rational(0, 1))   (Rational(0, 1))  (Rational(0, 1)) (Rational(0, 1));
            ] as &dyn Mat<Item = _>
        );
    }

    #[test]
    fn test_simplify() {
        let mut b = eliminate();
        b.simplify();

        assert_eq!(&b as &dyn Mat<Item=Rational>, &mat_![
            (Rational(1, 1))  (Rational(1, 2))  (Rational(-1, 2))  (Rational(1, 2)) (Rational(1, 2));
            (Rational(0, 1))  (Rational(1, 1)) (Rational(-5, 7))  (Rational(9, 7))  (Rational(-5, 7));
            (Rational(0, 1))  (Rational(0, 1))   (Rational(0, 1))  (Rational(0, 1)) (Rational(0, 1));
        ].convert() as &dyn Mat<Item=Rational>);
    }

    #[test]
    fn test_reduced() {
        let mut b = eliminate();
        b.reduce();

        assert_eq!(&b as &dyn Mat<Item=Rational>, &mat_![
            (Rational(1, 1))  (Rational(0, 1))  (Rational(-1, 7))  (Rational(-1, 7)) (Rational(6, 7));
            (Rational(0, 1))  (Rational(1, 1)) (Rational(-5, 7))  (Rational(9, 7))  (Rational(-5, 7));
            (Rational(0, 1))  (Rational(0, 1))   (Rational(0, 1))  (Rational(0, 1)) (Rational(0, 1));
        ].convert() as &dyn Mat<Item=Rational>);
    }

    #[test]
    fn test_rank() {
        let b = eliminate();
        assert_eq!(b.rank(), 2);
    }

    #[test]
    fn test_rank2() {
        let m = mat_![1 2; 3 4;];
        let m = m.eliminated();
        assert_eq!(m.rank(), 2)
    }

    #[test]
    fn test_null_space() {
        let a: DataMatrix<i32> = mat_![
            1 2 1 4;
            0 1 3 1;
            0 0 0 0;
        ];
        let b = a.eliminated();
        let n = b.null_space().unwrap();

        assert_eq!(
            n,
            mat_![
                -2  5;
                -1 -3;
                 0  1;
                 1  0;
            ]
        );
    }

    #[test]
    fn test_special_solution() {
        let a: DataMatrix<i32> = mat_![
            1 2 1 4;
            0 1 3 1;
            0 0 0 0;
        ];
        let a = a.eliminated();
        let b: DataMatrix<i32> = mat_![
            1;
            2;
            0;
        ];

        let special_solution = a.special_solution(&b).unwrap();
        assert_eq!(
            special_solution,
            mat_![
                -3;
                2;
                0;
                0;
            ]
        );
    }
}
