use super::{DataMatrix, Mat};
use crate::error::MatError;
use MatError::*;
use mat_macro::concated_mat_;
use crate::element::*;

/// Indicates what solution the equation has
pub enum SolveResult<T> where T: LinearElem {
    /// No solution
    None,
    /// Single special solution
    Single(DataMatrix<T>),
    /// Infinite amount of solution composed of general solutions and a special solution
    /// 
    /// General solutions are stored in cols of matrix `general`
    Infinite {
        general: DataMatrix<T>,
        special: DataMatrix<T>
    },
}

mod display {
    use super::super::display::MatPrint;

    use super::*;
    use std::fmt::Display;

    impl<T> Display for SolveResult<T> where T: LinearElem + Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SolveResult::None => write!(f, "None"),
                SolveResult::Single(sol) => sol.mat_print_fmt(f),
                SolveResult::Infinite { general, special }
                => {
                    write!(f, "Special:\n")?;
                    special.mat_print_fmt(f)?;
                    write!(f, "General:\n")?;
                    general.mat_print_fmt(f)
                }
            }
        }
    }

}


pub fn solve_augmented<T: LinearElem>(augmented: impl Mat<Item=T>) -> SolveResult<T> {

    let augmented = augmented.eliminated();

    let coef_slice = augmented.slice(
        0, augmented.rows(), 0, augmented.cols() - 1
    );
    let b_slice = augmented.slice(
        0, augmented.rows(), augmented.cols() - 1, 1
    );

    let special = coef_slice.special_solution(&b_slice);

    match special {
        None => SolveResult::None,
        Some(special) => {
            match coef_slice.null_space() {
                None => SolveResult::Single(special),
                Some(general) => SolveResult::Infinite { general, special }
            }
        }
    }

}

pub fn solve<T: LinearElem>(
    coef: &mut dyn Mat<Item=T>, b: &mut dyn Mat<Item=T>
) -> Result<SolveResult<T>, MatError> {

    if b.cols() != 1 {
        return Err(InconsistentDimension { need: (b.rows(), 1), got: b.dimensions() });
    }

    let augmented = concated_mat_![coef b;]?;

    Ok(solve_augmented(augmented))
}

#[cfg(test)]
mod test {
    use super::*;
    use mat_macro::mat_;


    #[test]
    fn test_none() {
        let mut a = mat_![
            1 2;
            0 0;
        ].unwrap();
        let mut b = mat_![
            1;
            1;
        ].unwrap();

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => (),
            Single(_) => panic!("Got single solution"),
            Infinite { general: _, special: _} => panic!("Got infininte solution"),
        }
    }

    #[test]
    fn test_single() {
        let mut a = mat_![
            1 2;
            0 1;
        ].unwrap();
        let mut b = mat_![
            1;
            1;
        ].unwrap();

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(sol)
                => assert_eq!(sol, mat_![-1; 1;].unwrap()),
            Infinite { general: _, special: _} => panic!("Got infininte solution"),
        }
    }

    #[test]
    fn test_infinite() {
        let mut a = mat_![
            1 2 1;
            0 1 1;
        ].unwrap();
        let mut b = mat_![
            1;
            1;
        ].unwrap();

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(_) => panic!("Got single solution"),
            Infinite { general, special } => {
                assert_eq!(general, mat_![ 1; -1; 1; ].unwrap());
                assert_eq!(special, mat_![ -1; 1; 0; ].unwrap());
            },
        }
    }
}
