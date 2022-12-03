use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum MatError {
    /// Index exceeds dimension of the matrix: getting (i, j) of a mat with (rows, cols)
    IndexError {
        dim: (usize, usize),
        i: usize,
        j: usize,
        mutable: bool,
    },
    /// When operation on matrixs, the dimension doesn't satisfy the requirement of the operation
    InconsistentDimension {
        need: (usize, usize),
        got: (usize, usize),
    },
    /// Can't be inverted
    NotInvertable { rank: usize, rows: usize },
    /// Initialization vector length is unindentical with rows and cols
    BadInitVector {
        len: usize,
        cols: usize,
        rows: usize,
    },
    /// not a square
    NotSquare { dim: (usize, usize) },
    /// Can't concat
    ConcatFailure(String),
    /// Empty
    EmptyMatrix,
}

impl Display for MatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MatError::*;

        match self {
            IndexError { dim, i, j, mutable } => write!(
                f,
                "Can't {} ({},{}) of a {:?} mat (or row or col)",
                if *mutable { "alter" } else { "read" },
                i,
                j,
                dim
            ),
            InconsistentDimension { need, got } => {
                write!(f, "Need dimension {:?}, got {:?}", need, got)
            }
            NotInvertable { rank, rows } => write!(
                f,
                "Can't invert a ({},{}) matrix with rank {}",
                rows, rows, rank
            ),
            BadInitVector { len, cols, rows } => write!(
                f,
                "To init a ({},{}) matrix need a {} vec, not {}",
                rows,
                cols,
                rows * cols,
                len
            ),
            NotSquare { dim } => write!(f, "Need a square matrix, got {:?}", dim),
            ConcatFailure(s) => write!(f, "{s}"),
            EmptyMatrix => write!(f, "Empty matrix"),
        }
    }
}

impl Error for MatError {}
