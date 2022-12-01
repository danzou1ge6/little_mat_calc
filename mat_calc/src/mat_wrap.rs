use mat::{DataMatrix, Mat, Rational};

/// Wrap two types of matrix: [`Rational`] and [`f64`] ,
/// These are the only two types supported
#[derive(Debug)]
pub enum MatrixWrap {
    Rat(Box<dyn Mat<Item = Rational>>),
    Flt(Box<dyn Mat<Item = f64>>),
}

impl Clone for MatrixWrap {
    fn clone(&self) -> Self {
        match self {
            MatrixWrap::Flt(m) => MatrixWrap::Flt(Box::new(m.clone_data())),
            MatrixWrap::Rat(m) => MatrixWrap::Rat(Box::new(m.clone_data())),
        }
    }
}

mod error {
    #[derive(Debug)]
    pub struct ParseMatrixError(pub String);
}
pub use error::ParseMatrixError;

impl TryInto<MatrixWrap> for &mut dyn Iterator<Item = &str> {
    type Error = ParseMatrixError;
    fn try_into(self) -> Result<MatrixWrap, Self::Error> {
        let mut rows = 0;
        let mut cols = 0;
        let mut last_cols = 0;
        let mut float = false;

        let mut rats: Vec<Rational> = Vec::new();
        let mut floats: Vec<f64> = Vec::new();

        while let Some(piece) = self.next() {
            match piece {
                ";" => {
                    rows += 1;
                    if last_cols != 0 && cols != last_cols {
                        return Err(ParseMatrixError(format!("Inconsistent columns")));
                    }
                    last_cols = cols;
                    cols = 0;
                }
                other => {
                    if let Ok(rat) = other.try_into() {
                        if !float {
                            rats.push(rat);
                        } else {
                            floats.push(rat.into());
                        }
                    } else {
                        if !float {
                            floats = rats.iter().map(|&x| x.into()).collect();
                            float = true;
                        }
                        if let Ok(flt) = other.parse() {
                            floats.push(flt);
                        } else {
                            return Err(ParseMatrixError(format!("Can't parse {}", other)));
                        }
                    }
                    cols += 1;
                }
            }
        }
        if cols != 0 {
            rows += 1;
            if last_cols != 0 && cols != last_cols {
                return Err(ParseMatrixError(format!("Inconsistent columns")));
            }
            last_cols = cols;
        }
        cols = last_cols;

        if float {
            return Ok(MatrixWrap::Flt(Box::new(
                DataMatrix::new(floats, rows, cols).unwrap(),
            )));
        } else {
            return Ok(MatrixWrap::Rat(Box::new(
                DataMatrix::new(rats, rows, cols).unwrap(),
            )));
        }
    }
}

mod display {
    use super::*;
    use mat::matrix::mat_print_buf;
    use std::fmt::Display;

    impl Display for MatrixWrap {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use MatrixWrap::*;
            match self {
                Rat(m) => mat_print_buf(m.as_ref(), f),
                Flt(m) => mat_print_buf(m.as_ref(), f),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mat::mat;

    #[test]
    fn test_parse() {
        let mw: MatrixWrap = (&mut vec!["1", "1", ";", "2/1", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        match mw {
            MatrixWrap::Flt(_) => panic!("should be rational"),
            MatrixWrap::Rat(r) => assert_eq!(
                r.as_ref() as &dyn Mat<Item = Rational>,
                &mat![1 1; 2 3;].convert() as &dyn Mat<Item = Rational>
            ),
        }
    }

    #[test]
    fn test_parse_float() {
        let mw: MatrixWrap = (&mut vec!["1", "1/1", ";", "2.5", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        match mw {
            MatrixWrap::Rat(_) => panic!("should be float"),
            MatrixWrap::Flt(f) => assert_eq!(
                f.as_ref() as &dyn Mat<Item = f64>,
                &mat![1.0 1.0; 2.5 3.0;].convert() as &dyn Mat<Item = f64>
            ),
        }
    }
}
