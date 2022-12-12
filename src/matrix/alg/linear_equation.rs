use crate::element::*;
use crate::error::MatError;
use crate::matrix::{DataMatrix, Mat};
use mat_macro::concated_mat_;
use MatError::*;

/// Indicates what solution the equation has
pub enum SolveResult<T>
where
    T: LinearElem,
{
    /// No solution
    None,
    /// Single special solution
    Single(DataMatrix<T>),
    /// Infinite amount of solution composed of general solutions and a special solution
    ///
    /// General solutions are stored in cols of matrix `general`
    Infinite {
        general: DataMatrix<T>,
        special: DataMatrix<T>,
    },
}

mod display {
    use super::*;
    use crate::matrix::mat_print_buf;
    use std::fmt::Display;

    impl<T> Display for SolveResult<T>
    where
        T: LinearElem + Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SolveResult::None => write!(f, "None"),
                SolveResult::Single(sol) => mat_print_buf(sol, f),
                SolveResult::Infinite { general, special } => {
                    write!(f, "Special:\n")?;
                    mat_print_buf(special, f)?;
                    write!(f, "General:\n")?;
                    mat_print_buf(general, f)
                }
            }
        }
    }
}

pub fn solve_augmented<T: LinearElem>(augmented: impl Mat<Item = T>) -> SolveResult<T>
where
    T: LinearElem + Inv,
{
    let augmented = augmented.eliminated();

    let coef_slice = augmented.slice(0, augmented.rows(), 0, augmented.cols() - 1);
    let b_slice = augmented.slice(0, augmented.rows(), augmented.cols() - 1, 1);

    let special = coef_slice.special_solution(&b_slice);

    match special {
        None => SolveResult::None,
        Some(special) => match coef_slice.null_space() {
            None => SolveResult::Single(special),
            Some(general) => {
                SolveResult::Infinite { general, special }
            },
        },
    }
}

/// Solve linear equation; supports occassion of infinite solution.
/// Caution that the inputed matrixes are ruined by elimination
pub fn solve<'a, T>(
    coef: &'a mut dyn Mat<Item = T>,
    b: &'a mut dyn Mat<Item = T>,
) -> Result<SolveResult<T>, MatError>
where
    T: LinearElem + Inv,
{
    if b.cols() != 1 {
        return Err(InconsistentDimension {
            need: (b.rows(), 1),
            got: b.dimensions(),
        });
    }

    let augmented = concated_mat_![coef b;]?;

    Ok(solve_augmented(augmented))
}

#[cfg(test)]
mod test {
    use crate::Rational;

    use super::*;
    use mat_macro::mat_;

    #[test]
    fn test_none() {
        let mut a = mat_![
            1 2;
            0 0;
        ];
        let mut b = mat_![
            1;
            1;
        ];

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => (),
            Single(_) => panic!("Got single solution"),
            Infinite {
                general: _,
                special: _,
            } => panic!("Got infininte solution"),
        }
    }

    #[test]
    fn test_single() {
        let mut a = mat_![
            1 2;
            0 1;
        ];
        let mut b = mat_![
            1;
            1;
        ];

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(sol) => assert_eq!(sol, mat_![-1; 1;]),
            Infinite {
                general: _,
                special: _,
            } => panic!("Got infininte solution"),
        }
    }

    #[test]
    fn test_infinite() {
        let mut a: DataMatrix<Rational> = mat_![
            1 2 1;
            0 1 1;
        ].convert();
        let mut b: DataMatrix<Rational> = mat_![
            1;
            1;
        ].convert();

        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(_) => panic!("Got single solution"),
            Infinite { general, special } => {
                assert_eq!(general, mat_![ 1; -1; 1; ].convert());
                assert_eq!(special, mat_![ -1; 1; 0; ].convert());
            }
        }
    }

    #[test]
    fn test_infinite2() {
        let mut a: DataMatrix<Rational> = mat_![3 4 5; 9 3 2;].convert();
        let mut b: DataMatrix<Rational> = mat_![1; 4;].convert();
        
        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(_) => panic!("Got single solution"),
            Infinite { general, special } => {
                assert_eq!(special, mat_![ (Rational(13, 27)); (Rational(-1, 9)); (Rational(0, 1)); ]);
                assert_eq!(general, mat_![ (Rational(7, 27)); (Rational(-13, 9)); (Rational(1, 1)); ]);
            }
        }
    }

    #[test]
    fn test_infinite3() {
        let mut a: DataMatrix<Rational> = mat_![3 4 5; 9 3 2; 12 7 7;].convert();
        let mut b: DataMatrix<Rational> = mat_![1; 4; 5;].convert();
        
        use SolveResult::*;
        match solve(&mut a, &mut b).unwrap() {
            None => panic!("No solution"),
            Single(_) => panic!("Got single solution"),
            Infinite { general, special } => {
                assert_eq!(special, mat_![ (Rational(13, 27)); (Rational(-1, 9)); (Rational(0, 1)); ]);
                assert_eq!(general, mat_![ (Rational(7, 27)); (Rational(-13, 9)); (Rational(1, 1)); ]);
            }
        }
    }
}
