use crate::table::Table;
use mat::{DataMatrix, Mat, Rational, rational, Complex};
use std::rc::Rc;

/// Wrap two types of matrix: [`Rational`] and [`f64`] , and also the symbol table [`Table<Token>`]
/// These are the only two types supported
#[derive(Debug)]
pub enum MatrixWrap {
    Rat(Rc<dyn Mat<Item = Rational>>),
    Cpl(Rc<dyn Mat<Item = Complex>>),
}

pub enum MatrixOrTable {
    Matrix(MatrixWrap),
    Table(Table<String>),
}

impl MatrixOrTable {
    pub fn matrix(self) -> Option<MatrixWrap> {
        match self {
            MatrixOrTable::Matrix(m) => Some(m),
            MatrixOrTable::Table(_) => None,
        }
    }

    pub fn table(self) -> Option<Table<String>> {
        match self {
            MatrixOrTable::Matrix(_) => None,
            MatrixOrTable::Table(t) => Some(t),
        }
    }
}

impl Clone for MatrixWrap {
    fn clone(&self) -> Self {
        match self {
            MatrixWrap::Cpl(m) => MatrixWrap::Cpl(Rc::new(m.clone_data())),
            MatrixWrap::Rat(m) => MatrixWrap::Rat(Rc::new(m.clone_data())),
        }
    }
}

mod error {
    #[derive(Debug)]
    pub struct ParseMatrixError(pub String);
}
pub use error::ParseMatrixError;

#[derive(PartialEq, Eq)]
enum ParsingMode {
    Rational,
    Complex,
    Symbol,
    None,
}

impl TryInto<MatrixOrTable> for &mut dyn Iterator<Item = &str> {
    type Error = ParseMatrixError;
    fn try_into(self) -> Result<MatrixOrTable, Self::Error> {
        let mut rows = 0;
        let mut cols = 0;
        let mut last_cols = 0;

        let mut parsing_mode = ParsingMode::None;

        let mut rats: Vec<Rational> = Vec::new();
        let mut complexes: Vec<Complex> = Vec::new();
        let mut words: Vec<String> = Vec::new();

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
                    // if rational
                other => {
                    match other.try_into() {
                        Err(rational::ParseError::ZeroDivision) =>
                            // zero division
                            return Err(ParseMatrixError(format!("Can't devide by zero"))),
                        Ok(rat) => {
                            // rational
                            match parsing_mode {
                                ParsingMode::Rational => rats.push(rat),
                                ParsingMode::Complex => {
                                    let flt: f64 = rat.into();
                                    complexes.push(flt.into());
                                }
                                ParsingMode::Symbol => {
                                    return Err(ParseMatrixError(format!(
                                        "Symbol table doesn't accept rational"
                                    )))
                                }
                                ParsingMode::None => {
                                    parsing_mode = ParsingMode::Rational;
                                    rats.push(rat);
                                }
                            }
                        },
                        Err(rational::ParseError::NotARational) => {
                            // if complex
                            if let Ok(cpl) = other.try_into() {
                                match parsing_mode {
                                    ParsingMode::Complex => complexes.push(cpl),
                                    ParsingMode::Rational => {
                                        complexes = rats.iter().map(|&x| f64::from(x).into()).collect();
                                        complexes.push(cpl);
                                        parsing_mode = ParsingMode::Complex;
                                    }
                                    ParsingMode::Symbol => {
                                        return Err(ParseMatrixError(format!(
                                            "Symbol table doesn't accept complex"
                                        )))
                                    }
                                    ParsingMode::None => {
                                        parsing_mode = ParsingMode::Complex;
                                        complexes.push(cpl);
                                    }
                                }
                            // if symbol table
                            } else {
                                match parsing_mode {
                                    ParsingMode::Symbol => words.push(other.to_string()),
                                    ParsingMode::None => {
                                        parsing_mode = ParsingMode::Symbol;
                                        words.push(other.to_string());
                                    }
                                    _ => {
                                        return Err(ParseMatrixError(format!(
                                            "Symbol table doesn't accept complex or rational"
                                        )))
                                    }
                                }
                            }
                        },
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

        match parsing_mode {
            ParsingMode::Complex => {
                return Ok(MatrixOrTable::Matrix(MatrixWrap::Cpl(Rc::new(
                    DataMatrix::new(complexes, rows, cols).unwrap(),
                ))))
            }
            ParsingMode::Rational => {
                return Ok(MatrixOrTable::Matrix(MatrixWrap::Rat(Rc::new(
                    DataMatrix::new(rats, rows, cols).unwrap(),
                ))))
            }
            ParsingMode::Symbol => {
                return Ok(MatrixOrTable::Table(Table::new(words, rows, cols).unwrap()));
            }
            ParsingMode::None => return Err(ParseMatrixError(format!("Empty matrix not allowed"))),
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
                Cpl(m) => mat_print_buf(m.as_ref(), f),
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
        let mw: MatrixOrTable = (&mut vec!["1", "1", ";", "2/1", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        match mw.matrix().unwrap() {
            MatrixWrap::Rat(r) => assert_eq!(
                r.as_ref() as &dyn Mat<Item = Rational>,
                &mat![1 1; 2 3;].convert() as &dyn Mat<Item = Rational>
            ),
            _ => panic!("should be rational"),
        }
    }

    #[test]
    fn test_parse_complex() {
        let mw: MatrixOrTable = (&mut vec!["1", "1/1", ";", "2.5", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        match mw.matrix().unwrap() {
            MatrixWrap::Cpl(f) => assert_eq!(
                f.as_ref() as &dyn Mat<Item = Complex>,
                &mat![1.0 1.0; 2.5 3.0;].convert() as &dyn Mat<Item = Complex>
            ),
            _ => panic!("should be complex"),
        }
    }

    #[test]
    fn test_parse_symbol() {
        let mw: MatrixOrTable = (&mut vec!["a", "a", ";", "a", "a", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        let t = mw.table().unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(t.get(i, j).unwrap(), "a");
            }
        }
    }
}
