use mat::{DataMatrix, Mat, Rational};
use crate::{table::Table};

/// Wrap two types of matrix: [`Rational`] and [`f64`] , and also the symbol table [`Table<Token>`]
/// These are the only two types supported
#[derive(Debug)]
pub enum MatrixWrap {
    Rat(Box<dyn Mat<Item = Rational>>),
    Flt(Box<dyn Mat<Item = f64>>),
}

pub enum MatrixOrTable {
    Matrix(MatrixWrap),
    Table(Table<String>)
}

impl MatrixOrTable {
    pub fn matrix(self) -> Option<MatrixWrap> {
        match self {
            MatrixOrTable::Matrix(m) => Some(m),
            MatrixOrTable::Table(_) => None
        }
    }

    pub fn table(self) -> Option<Table<String>> {
        match self {
            MatrixOrTable::Matrix(m) => None,
            MatrixOrTable::Table(t) => Some(t)
        }
    }
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

#[derive(PartialEq, Eq)]
enum ParsingMode {
    Rational,
    Float,
    Symbol,
    None
}

impl TryInto<MatrixOrTable> for &mut dyn Iterator<Item = &str> {
    type Error = ParseMatrixError;
    fn try_into(self) -> Result<MatrixOrTable, Self::Error> {
        let mut rows = 0;
        let mut cols = 0;
        let mut last_cols = 0;

        let mut parsing_mode = ParsingMode::None;

        let mut rats: Vec<Rational> = Vec::new();
        let mut floats: Vec<f64> = Vec::new();
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
                other => {
                    // if rational
                    if let Ok(rat) = other.try_into() {
                        match parsing_mode {
                            ParsingMode::Rational => rats.push(rat),
                            ParsingMode::Float => floats.push(rat.into()),
                            ParsingMode::Symbol => return Err(ParseMatrixError(format!("Symbol table doesn't accept rational"))),
                            ParsingMode::None => {
                                parsing_mode = ParsingMode::Rational;
                                rats.push(rat);
                            }
                        }
                    } else {
                        // if float
                        if let Ok(flt) = other.parse() {
                            match parsing_mode {
                                ParsingMode::Float => floats.push(flt),
                                ParsingMode::Rational => {
                                    floats = rats.iter().map(|&x| x.into()).collect();
                                    floats.push(flt);
                                    parsing_mode = ParsingMode::Float;
                                },
                                ParsingMode::Symbol => return Err(ParseMatrixError(format!("Symbol table doesn't accept float"))),
                                ParsingMode::None => {
                                    parsing_mode = ParsingMode::Float;
                                    floats.push(flt);
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
                                _ => return Err(ParseMatrixError(format!("Symbol table doesn't accept float or rational"))),
                            }
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

        match parsing_mode {
            ParsingMode::Float => 
                return Ok(MatrixOrTable::Matrix(MatrixWrap::Flt(Box::new(
                    DataMatrix::new(floats, rows, cols).unwrap(),
                )))),
            ParsingMode::Rational => 
                return Ok(MatrixOrTable::Matrix(MatrixWrap::Rat(Box::new(
                    DataMatrix::new(rats, rows, cols).unwrap(),
                )))),
            ParsingMode::Symbol => {
                return Ok(MatrixOrTable::Table(Table::new(
                    words, rows, cols
                ).unwrap()));
            },
            ParsingMode::None => return Err(ParseMatrixError(format!("Empty matrix not allowed")))
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
        let mw: MatrixOrTable = (&mut vec!["1", "1", ";", "2/1", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into().unwrap();
        match mw.matrix().unwrap() {
            MatrixWrap::Rat(r) => assert_eq!(
                r.as_ref() as &dyn Mat<Item = Rational>,
                &mat![1 1; 2 3;].convert() as &dyn Mat<Item = Rational>
            ),
            _ => panic!("should be rational"),
        }
    }

    #[test]
    fn test_parse_float() {
        let mw: MatrixOrTable = (&mut vec!["1", "1/1", ";", "2.5", "3", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        match mw.matrix().unwrap() {
            MatrixWrap::Flt(f) => assert_eq!(
                f.as_ref() as &dyn Mat<Item = f64>,
                &mat![1.0 1.0; 2.5 3.0;].convert() as &dyn Mat<Item = f64>
            ),
            _ => panic!("should be float"),
        }
    }

    #[test]
    fn test_parse_symbol() {
        let mw: MatrixOrTable = (&mut vec!["a", "a", ";", "a", "a", ";"].iter().map(|x| *x)
            as &mut dyn Iterator<Item = &str>)
            .try_into()
            .unwrap();
        let t = mw.table().unwrap();
        for i in 0..2 { for j in 0..2 {
            assert_eq!(t.get(i, j).unwrap(), "a");
        }}
    }
}
