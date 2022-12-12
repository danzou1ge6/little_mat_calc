use crate::eval::BuiltinFunction;
use crate::eval::{Environment, ObjectPairItem};
use indoc::indoc;
use mat::{alg, SliceMatrix};
use mat::alg::SolveResult;
use mat::element::LinearElem;
use mat::element::Inv;
use mat::error::MatError;
use mat::ConcatedMatrix;
use mat::DataMatrix;
use mat::Mat;
use std::rc::Rc;

use super::Output;
use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPair;
use crate::eval::ObjectPairItem::*;
use crate::mat_wrap::MatrixWrap;

pub fn inv(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => match alg::inv(&mut m.clone_data()) {
            Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(r))))),
            Err(e) => return Err(EvalError::value(format!("{e}"))),
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => match alg::inv(&mut m.clone_data()) {
            Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(r))))),
            Err(e) => return Err(EvalError::value(format!("{e}"))),
        },
        Lit(Rat(r)) => return Ok(Lit(Rat(r.inv()))),
        Lit(Cplx(f)) => return Ok(Lit(Cplx(f.inv()))),
        other => return Err(EvalError::typ(format!("Can't inv {other}"))),
    }
}

pub fn eliminate(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let r = m.clone_data().eliminated();
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(r)))));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated();
            return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(r)))));
        }
        _ => return Err(EvalError::typ(format!("Can only eliminate a matrix"))),
    }
}

pub fn reduce(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(_) => {
            return Err(EvalError::syntax(
                "You can only gussian eliminate an item a time".to_string(),
            ))
        }
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let r = m.clone_data().eliminated().reduced();
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(r)))));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated().reduced();
            return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(r)))));
        }
        _ => return Err(EvalError::typ(format!("Can only eliminate a matrix"))),
    }
}

pub fn rank(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let r = m.clone_data().eliminated().rank() as i32;
            return Ok(Lit(Rat(r.into())));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated().rank() as i32;
            return Ok(Lit(Rat(r.into())));
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only calculate rank of a matrix"
            )))
        }
    }
}

pub fn det(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let d = match alg::det(m.as_ref()) {
                Ok(d) => d,
                Err(e) => return Err(EvalError::value(format!("{e}"))),
            };
            return Ok(Lit(Cplx(d)));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let d = match alg::det(m.as_ref()) {
                Ok(d) => d,
                Err(e) => return Err(EvalError::value(format!("{e}"))),
            };
            return Ok(Lit(Rat(d)));
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only calculate rank of a matrix"
            )))
        }
    }
}
pub fn solve(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(Matrix(MatrixWrap::Cpl(a))), Lit(Matrix(MatrixWrap::Cpl(b)))) => {
                let r = alg::solve(&mut a.clone_data(), &mut b.clone_data())
                    .map_err(|e| EvalError::value(format!("{e}")))?;
                match r {
                    SolveResult::None => return Ok(Lit(Nil)),
                    SolveResult::Single(s) => return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(s))))),
                    SolveResult::Infinite { general, special } => {
                        return Ok(List(Box::new(ObjectPair {
                            first: Lit(Matrix(MatrixWrap::Cpl(Rc::new(general)))),
                            second: Lit(Matrix(MatrixWrap::Cpl(Rc::new(special)))),
                        })));
                    }
                }
            }
            (Lit(Matrix(MatrixWrap::Rat(a))), Lit(Matrix(MatrixWrap::Rat(b)))) => {
                let r = alg::solve(&mut a.clone_data(), &mut b.clone_data())
                    .map_err(|e| EvalError::value(format!("{e}")))?;
                match r {
                    SolveResult::None => return Ok(Lit(Nil)),
                    SolveResult::Single(s) => return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(s))))),
                    SolveResult::Infinite { general, special } => {
                        return Ok(List(Box::new(ObjectPair {
                            first: Lit(Matrix(MatrixWrap::Rat(Rc::new(general)))),
                            second: Lit(Matrix(MatrixWrap::Rat(Rc::new(special)))),
                        })));
                    }
                }
            }
            (a, b) => {
                return Err(EvalError::syntax(format!(
                    "Need two matrixes of same type to solve, found {} and {}",
                    a, b
                )))
            }
        },
        _ => {
            return Err(EvalError::syntax(
                "Need two arguments to solve linear equation".to_string(),
            ))
        }
    }
}

pub fn transpose(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(
                m.clone_data().transposed(),
            )))));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(
                m.clone_data().transposed(),
            )))));
        }
        _ => return Err(EvalError::typ(format!("Can only transpose a matrix"))),
    }
}

pub fn trace(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            return Ok(Lit(Cplx(
                alg::trace(m.as_ref()).map_err(|e| EvalError::value(format!("{e}")))?,
            )));
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            return Ok(Lit(Rat(
                alg::trace(m.as_ref()).map_err(|e| EvalError::value(format!("{e}")))?
            )));
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only calculate trace of a matrix"
            )))
        }
    }
}

pub fn null_space(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            if let Some(ns) = m.clone_data().eliminated().null_space() {
                return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(ns)))));
            } else {
                return Ok(Lit(Nil));
            }
        }
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            if let Some(ns) = m.clone_data().eliminated().null_space() {
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(ns)))));
            } else {
                return Ok(Lit(Nil));
            }
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only calculate null space of a matrix"
            )))
        }
    }
}

pub fn ridentity(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Rat(r)) => {
            if r.1 != 1 || r.0 < 0 {
                return Err(EvalError::value(
                    "Need an positive integer, not a fraction".to_string(),
                ));
            } else {
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(DataMatrix::identity(
                    r.0.try_into().unwrap(),
                ))))));
            }
        }
        _ => return Err(EvalError::typ(format!("Argument takes a postive integer"))),
    }
}
pub fn cidentity(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Rat(r)) => {
            if r.1 != 1 || r.0 < 0 {
                return Err(EvalError::value(
                    "Need an positive integer, not a fraction".to_string(),
                ));
            } else {
                return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(DataMatrix::identity(
                    r.0.try_into().unwrap(),
                ))))));
            }
        }
        _ => return Err(EvalError::typ(format!("Argument takes a positive integer"))),
    }
}

fn clone_concated<T>(
    data: Vec<&dyn Mat<Item = T>>,
    rows: usize,
    cols: usize,
) -> Result<DataMatrix<T>, MatError>
where
    T: LinearElem,
{
    let mut_data = unsafe {
        data.into_iter()
            .map(|x| {
                &mut *(x as *const dyn Mat<Item = _> as *mut dyn Mat<Item = _>)
                    as &mut dyn Mat<Item = _>
            })
            .collect()
    };
    let concated = ConcatedMatrix::new(mut_data, rows, cols)?;
    return Ok(concated.clone_data());
}

pub fn concat(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Table(t)) => {
            if t.data.len() == 0 {
                return Err(EvalError::value(format!("Empty matrix not allowed")));
            }

            match t.data[0] {
                Lit(Matrix(MatrixWrap::Cpl(_))) => {
                    let mut mt_data = Vec::with_capacity(t.data.len());
                    for o in t.data.iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Cpl(m))) => mt_data.push(m.as_ref()),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or complex)")))
                        }
                    }
                    let concated = clone_concated(mt_data, t.rows, t.cols)
                        .map_err(|e| EvalError::value(format!("{e}")))?;
                    return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(concated)))));
                }
                Lit(Matrix(MatrixWrap::Rat(_))) => {
                    let mut mt_data = Vec::with_capacity(t.data.len());
                    for o in t.data.iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Rat(m))) => mt_data.push(m.as_ref()),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or complex)")))
                        }
                    }
                    let concated = clone_concated(mt_data, t.rows, t.cols)
                        .map_err(|e| EvalError::value(format!("{e}")))?;
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(concated)))));
                }
                _ => return Err(EvalError::typ(format!("Can only concat matrixes"))),
            }
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only concat a table of matrixes"
            )))
        }
    }
}

pub fn get(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(box ObjectPair {
            first: Lit(Matrix(MatrixWrap::Cpl(m))),
            second:
                List(box ObjectPair {
                    first: Lit(Rat(i)),
                    second: Lit(Rat(j)),
                }),
        }) => {
            if i.1 != 1 || j.1 != 1 {
                return Err(EvalError::value(format!(
                    "Can only index into matrix by integers"
                )));
            }
            if let (Ok(ui), Ok(uj)) = (i.0.try_into(), j.0.try_into()) {
                return Ok(Lit(Cplx(
                    *m.get(ui, uj) ?,
                )));
            } else {
                return Err(EvalError::value(format!("Bad index")));
            }
        }
        List(box ObjectPair {
            first: Lit(Matrix(MatrixWrap::Rat(m))),
            second:
                List(box ObjectPair {
                    first: Lit(Rat(i)),
                    second: Lit(Rat(j)),
                }),
        }) => {
            if i.1 != 1 || j.1 != 1 {
                return Err(EvalError::value(format!(
                    "Can only index into matrix by integers"
                )));
            }
            if let (Ok(ui), Ok(uj)) = (i.0.try_into(), j.0.try_into()) {
                return Ok(Lit(Rat(*m
                    .get(ui, uj) ?)));
            } else {
                return Err(EvalError::value(format!("Bad index")));
            }
        }
        _ => {
            return Err(EvalError::syntax(format!(
                "Can only apply `get` on matrixe and indices must be integers"
            )))
        }
    }
}

pub fn qr(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let mut r: DataMatrix<f64> = m.clone_data().convert();
            let q = alg::qr(&mut r)?;
            return Ok(List(Box::new(ObjectPair {
                first: Lit(Matrix(MatrixWrap::Cpl(Rc::new(q.convert())))),
                second: Lit(Matrix(MatrixWrap::Cpl(Rc::new(r.convert()))))
            })));
        },
        _ => return Err(EvalError::typ(format!("Can only QR decomposite a complex matrix with real values")))
    }
}

pub fn eigenmat(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let m: DataMatrix<f64> = m.clone_data().convert();
            let solver = alg::EigenValueSolver::new(m)?;
            let m = solver.eigen_mat(1e-3, 999);
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(m.convert())))));
        },
        _ => return Err(EvalError::typ(format!("Can only QR decomposite a complex matrix with real values")))
    }
}

pub fn eigenvalues(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args{ 
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let m: DataMatrix<f64> = m.clone_data().convert();
            let solver = alg::EigenValueSolver::new(m)?;
            let eigen_vals = solver.eigen_values(1e-3, 999);
            let len = eigen_vals.len();
            let eigen_vals = DataMatrix::new(eigen_vals, len, 1).unwrap();
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(eigen_vals)))));
        },
        _ => return Err(EvalError::typ(format!("Can only calculate eigenvalues of a complex matrix with real values")))
    }
}

pub fn diag(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            if m.cols() == 1 || m.rows() == 1 {
                return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(DataMatrix::with_diag(m.clone_data().data()))))));
            } else {
                return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(DataMatrix::one_col(m.clone_diag()))))));
            }
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            if m.cols() == 1 || m.rows() == 1 {
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(DataMatrix::with_diag(m.clone_data().data()))))));
            } else {
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(DataMatrix::one_col(m.clone_diag()))))));
            }
        },
        _ => return Err(EvalError::typ(format!("Take one matrix as arguments")))
    }
}

pub fn dim(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            Ok(List(Box::new(ObjectPair {
                first: Lit(Rat(i32::try_from(m.rows()).unwrap().into())),
                second: Lit(Rat(i32::try_from(m.cols()).unwrap().into())) })))
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            Ok(List(Box::new(ObjectPair {
                first: Lit(Rat(i32::try_from(m.rows()).unwrap().into())),
                second: Lit(Rat(i32::try_from(m.cols()).unwrap().into())) })))
        },
        _ => Err(EvalError::typ(format!("Can only get dimension of a matrix")))
    }
}

pub fn slice(args: ObjectPairItem, _: &mut Environment) -> Output {
    let mut v = Vec::new();
    if let Some(()) = args.unpack(5, &mut v) {
        match [v.pop().unwrap(), v.pop().unwrap(), v.pop().unwrap(), v.pop().unwrap(), v.pop().unwrap()] {
            [
                Lit(Rat(cols)),
                Lit(Rat(cb)),
                Lit(Rat(rows)),
                Lit(Rat(rb)),
                Lit(Matrix(MatrixWrap::Cpl(m))),
            ] => {
                if rb.1 != 1 || rows.1 != 1 || cb.1 != 1 || cols.1 != 1 {
                    return Err(EvalError::typ(format!("Rows and cols need to be integers")));
                }
                let [rb, rows, cb, cols] = [rb, rows, cb, cols].map(|x| x.0.try_into().unwrap());
                let slice = SliceMatrix::new(m.as_ref(), rb, rows, cb, cols)
                    .map_err(|e| EvalError::value(format!("{e}")))?;
                
                return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(slice.clone_data())))))
            },
            [
                Lit(Rat(cols)),
                Lit(Rat(cb)),
                Lit(Rat(rows)),
                Lit(Rat(rb)),
                Lit(Matrix(MatrixWrap::Rat(m))),
            ] => {
                if rb.1 != 1 || rows.1 != 1 || cb.1 != 1 || cols.1 != 1 {
                    return Err(EvalError::typ(format!("Rows and cols need to be integers")));
                }
                let [rb, rows, cb, cols] = [rb, rows, cb, cols].map(|x| x.0.try_into().unwrap());
                let slice = SliceMatrix::new(m.as_ref(), rb, rows, cb, cols)
                    .map_err(|e| EvalError::value(format!("{e}")))?;
                
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(slice.clone_data())))))
            }
            _ => return Err(EvalError::syntax(format!("Need arguments: matrix, integer, integer, integet, integet")))
        }
    } else {
        return Err(EvalError::syntax(format!("Not enough arguments")))
    }
}

pub fn orthogonalize(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let mut ret = m.clone_data();
            alg::orthogonalize(&mut ret);
            return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(ret)))));
        },
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let mut ret = m.clone_data();
            alg::orthogonalize(&mut ret);
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(ret)))));
        },
        _ => return Err(EvalError::typ(format!("Need a matrix as argument")))
    }
}

pub fn normalize_cols(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Cpl(m))) => {
            let ret = m.clone_data();
            let mut ret: DataMatrix<f64> = ret.convert();
            alg::normalize_cols(&mut ret);
            return Ok(Lit(Matrix(MatrixWrap::Cpl(Rc::new(ret.convert())))));
        },
        _ => return Err(EvalError::typ(format!("Need a complex matrix as argument")))
    }
 
}

pub const EXPORTS: [BuiltinFunction; 21] = [
    BuiltinFunction {
        f: &inv,
        argn: 1,
        name: "inv",
        help: indoc! {"
            Calculate the inversion of a INVERTIBLE matrix;
            Or 1/x if x is a number.  "},
    },
    BuiltinFunction {
        f: &eliminate,
        argn: 1,
        name: "eliminate",
        help: "Apply gussian elimination on the matrix.",
    },
    BuiltinFunction {
        f: &rank,
        argn: 1,
        name: "rank",
        help: "Calculate rank of a matrix",
    },
    BuiltinFunction {
        f: &det,
        argn: 1,
        name: "det",
        help: "Calculate determinant of a matrix",
    },
    BuiltinFunction {
        f: &solve,
        argn: 2,
        name: "solve",
        help: indoc! {"
            Usage: (solve A b) -> nil | (matrix matrix)
            Solve linear equation `Ax=b`, returning
            - nil if there is no solution
            - a one-column matrix if there is only one solution
            - `(general special)` where both are matrixes "},
    },
    BuiltinFunction {
        f: &transpose,
        argn: 1,
        name: "tp",
        help: "Transpose a matrix.",
    },
    BuiltinFunction {
        f: &reduce,
        argn: 1,
        name: "rref",
        help: "Calculate the Reduced Upper Echolon Form of a matrix",
    },
    BuiltinFunction {
        f: &trace,
        argn: 1,
        name: "tr",
        help: "Calculate the trace of a matrix",
    },
    BuiltinFunction {
        f: &null_space,
        argn: 1,
        name: "nspace",
        help: indoc! {"
            Usage: (nspace x: matrix) -> nil | matrix
            Calculates the null space of a matrix, returning
            - nil if the null space only consists of {0}
            - a matrix containing a basis for the null space "},
    },
    BuiltinFunction {
        f: &ridentity,
        argn: 1,
        name: "ri",
        help: "Returns a rational identity matrix of given row number",
    },
    BuiltinFunction {
        f: &cidentity,
        argn: 1,
        name: "ci",
        help: "Returns a complex (float) identity matrix of given row number",
    },
    BuiltinFunction {
        f: &concat,
        argn: 1,
        name: "concat",
        help: indoc! {"
            Usage: (concat t: table)
            Concat matrixes in the partition defined by `t`.
            `t` can be, for example, `[a b;]`, which join `b` to the right of `a`.  "},
    },
    BuiltinFunction {
        f: &get,
        name: "get",
        argn: 3,
        help: indoc! {"
            Usage: (get m: matrix i: rational j: rational)
            Get the `(i, j)` element of matrix `m`.  "},
    },
    BuiltinFunction {
        f: &qr,
        name: "qr",
        argn: 1,
        help: indoc! {"
            Calculate the QR decomposition of a matrix.
            The matrix must have data type complex, but this algorithim can't handle
            complex matrixes, so the input matrix is cast into real matrix by taking
            the real part of each element.
            Returns `(Q R)`."}
    },
    BuiltinFunction {
        f: &eigenmat,
        name: "eigmat",
        argn: 1,
        help: indoc! {"
            Calculate the eigenvalues of a matrix. The matrix must be complex, 
            but only the real part of each element is taken due to limitation of
            the algorithim.
            This function returns a matrix, where elements below the diagnol are
            zeros, and any 1x1 block on the diagnol is a real eigenvalue, and
            any 2x2 block on the diagnol represents two adjoint complex eigenvalues"}
    },
    BuiltinFunction {
        f: &eigenvalues,
        name: "eigval",
        argn: 1,
        help: indoc! {"
            Calculate and return the eigenvalues of a matrix in the form of a column
            vector.
            The matrix must be complex, but only the real part of each element is
            taken."}
    },
    BuiltinFunction {
        f: &diag,
        name: "diag",
        argn: 1,
        help: indoc! {"
            Usage: (diag m: matrix) -> matrix
            If `m` is one column or one row, then it's data is used to initialize
            a square matrix with diagnol from `m`;
            Otherwise, the diagnol of `m` is taken and returned as a column vector."}
    },
    BuiltinFunction {
        f: &dim,
        name: "dim",
        argn: 1,
        help: indoc! {"
            Get the dimension of a matrix, returning `(rows cols)`"}
    },
    BuiltinFunction {
        f: &slice,
        name: "slice",
        argn: 5,
        help: indoc! {"
            Usage: (slice m: matrix row-begin rows col-begin cols) -> matrix
            Get the slice of a matrix"}
    },
    BuiltinFunction {
        f: &orthogonalize,
        name: "ortho",
        argn: 1,
        help: indoc! {"
            Apply Schmidt procedure on the matrix's columns, but not normalizing the matrix."}
    },
    BuiltinFunction {
        f: &normalize_cols,
        name: "normalize",
        argn: 1,
        help: indoc! {"
            Normalize the columns of a float matrix, which is represented using the
            real part of a complex matrix"}
    }
];
