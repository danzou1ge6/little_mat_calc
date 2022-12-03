use crate::eval::BuiltinFunction;
use crate::eval::{Environment, ObjectPairItem};
use indoc::indoc;
use mat::alg;
use mat::alg::SolveResult;
use mat::element::LinearElem;
use mat::element::RefInv;
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
        Lit(Matrix(MatrixWrap::Flt(m))) => match alg::inv(&mut m.clone_data()) {
            Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(r))))),
            Err(e) => return Err(EvalError::value(format!("{e}"))),
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => match alg::inv(&mut m.clone_data()) {
            Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(r))))),
            Err(e) => return Err(EvalError::value(format!("{e}"))),
        },
        Lit(Rat(r)) => return Ok(Lit(Rat(r.inv()))),
        Lit(Float(f)) => return Ok(Lit(Float(f.inv()))),
        other => return Err(EvalError::typ(format!("Can't inv {other}"))),
    }
}

pub fn eliminate(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let r = m.clone_data().eliminated();
            return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(r)))));
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let r = m.clone_data().eliminated().reduced();
            return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(r)))));
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let d = match alg::det(m.as_ref()) {
                Ok(d) => d,
                Err(e) => return Err(EvalError::value(format!("{e}"))),
            };
            return Ok(Lit(Float(d)));
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
            (Lit(Matrix(MatrixWrap::Flt(a))), Lit(Matrix(MatrixWrap::Flt(b)))) => {
                let r = alg::solve(&mut a.clone_data(), &mut b.clone_data())
                    .map_err(|e| EvalError::value(format!("{e}")))?;
                match r {
                    SolveResult::None => return Ok(Lit(Nil)),
                    SolveResult::Single(s) => return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(s))))),
                    SolveResult::Infinite { general, special } => {
                        return Ok(List(Box::new(ObjectPair {
                            first: Lit(Matrix(MatrixWrap::Flt(Rc::new(general)))),
                            second: Lit(Matrix(MatrixWrap::Flt(Rc::new(special)))),
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            return Ok(Lit(Float(
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
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            if let Some(ns) = m.clone_data().eliminated().null_space() {
                return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(ns)))));
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
        _ => return Err(EvalError::typ(format!("Can only transpose a matrix"))),
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
                Lit(Matrix(MatrixWrap::Flt(_))) => {
                    let mut mt_data = Vec::with_capacity(t.data.len());
                    for o in t.data.iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Flt(m))) => mt_data.push(m.as_ref()),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or float)")))
                        }
                    }
                    let concated = clone_concated(mt_data, t.rows, t.cols)
                        .map_err(|e| EvalError::value(format!("{e}")))?;
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(concated)))));
                }
                Lit(Matrix(MatrixWrap::Rat(_))) => {
                    let mut mt_data = Vec::with_capacity(t.data.len());
                    for o in t.data.iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Rat(m))) => mt_data.push(m.as_ref()),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or float)")))
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
            first: Lit(Matrix(MatrixWrap::Flt(m))),
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
                return Ok(Lit(Float(
                    *m.get(ui, uj)
                        .map_err(|e| EvalError::value(format!("{e}")))?,
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
                    .get(ui, uj)
                    .map_err(|e| EvalError::value(format!("{e}")))?)));
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

pub const EXPORTS: [BuiltinFunction; 12] = [
    BuiltinFunction {
        f: &inv,
        argn: 1,
        name: "inv",
        help: indoc! {"
            Calculate the inversion of a INVERTIBLE matrix;
            Or 1/x if x is a number.
        "},
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
            - `(general special)` where both are matrixes
        "},
    },
    BuiltinFunction {
        f: &transpose,
        argn: 1,
        name: "transpose",
        help: "Transpose a matrix.",
    },
    BuiltinFunction {
        f: &reduce,
        argn: 1,
        name: "reduce",
        help: "Calculate the Reduced Upper Echolon Form of a matrix",
    },
    BuiltinFunction {
        f: &trace,
        argn: 1,
        name: "trace",
        help: "Calculate the trace of a matrix",
    },
    BuiltinFunction {
        f: &null_space,
        argn: 1,
        name: "trace",
        help: indoc! {"
            Usage: (nullspace x: matrix) -> nil | matrix
            Calculates the null space of a matrix, returning
            - nil if the null space only consists of {0}
            - a matrix containing a basis for the null space
        "},
    },
    BuiltinFunction {
        f: &ridentity,
        argn: 1,
        name: "ridentity",
        help: "Returns a rational identity matrix of given row number",
    },
    BuiltinFunction {
        f: &concat,
        argn: 1,
        name: "concat",
        help: indoc! {"
            Usage: (concat t: table)
            Concat matrixes in the partition defined by `t`.
            `t` can be, for example, `[a b;]`, which join `b` to the right of `a`.
        "},
    },
    BuiltinFunction {
        f: &get,
        name: "get",
        argn: 3,
        help: indoc! {"
            Usage: (get m: matrix i: rational j: rational)
            Get the `(i, j)` element of matrix `m`.
        "},
    },
];
