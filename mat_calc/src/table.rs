
pub struct Table<T> {
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

mod error {
    #[derive(Debug)]
    pub enum TableError {
        BadInit { data_size: usize, rows: usize, cols: usize },
        IndexError { dim: (usize, usize), i: usize, j: usize },
    }
    
    use std::fmt::Display;
    impl Display for TableError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::BadInit { data_size, rows, cols } => {
                    write!(f, "Init Table({},{}) with data of size {}", rows, cols, data_size)
                },
                Self::IndexError { dim, i, j } => {
                    write!(f, "Can't get ({},{}) of Table{:?}", i, j, dim)
                }
            }
        }
    }
}
use self::error::TableError;
use TableError::*;

impl<T> Table<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Result<Self, TableError> {
        if data.len() != rows * cols {
            return Err(TableError::BadInit { data_size: data.len(), rows, cols });
        }
        Ok(Table {
            data, rows, cols
        })
    }

    pub fn dimensions(&self) -> (usize, usize) { (self.rows, self.cols) }


    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        self.data.get_unchecked(i * self.cols + j)
    }
    pub unsafe fn get_mut_unchecked(&mut self, i: usize, j: usize) -> &mut T{
        self.data.get_unchecked_mut(i * self.cols + j)
    }

    /// Get reference of (i, j), checking if the index is in range
    pub fn get(&self, i: usize, j: usize) -> Result<&T, TableError> {
        if i >= self.rows || j >= self.cols {
            return Err(IndexError{dim: self.dimensions(), i, j});
        }
        return unsafe { Ok(self.get_unchecked(i, j)) };
    }
    /// Get mutable reference of (i, j), checking index
    pub fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, TableError> {
        if i >= self.rows || j >= self.cols {
            return Err(IndexError{dim: self.dimensions(), i, j});
        }
        return unsafe { Ok(self.get_mut_unchecked(i, j)) };
    
    }

 
}

impl<T> Clone for Table<T> where T: Clone {
    fn clone(&self) -> Self {
        Table {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols
        }
    }
}

mod display {
    use std::fmt::{Display, Debug};

    use super::*;

    impl<T> Display for Table<T> where T: Display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for i in 0..self.rows { for j in 0..self.cols {
                write!(f, "{}  ", self.get(i, j).unwrap())?;
            }}
            write!(f, "")
        }
    }

    impl<T> Debug for Table<T> where T: Debug {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for i in 0..self.rows { for j in 0..self.cols {
                write!(f, "{:?}  ", self.get(i, j).unwrap())?;
            }}
            write!(f, "")
        }
    }
}
