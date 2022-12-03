use crate::eval::{Environment, ObjectPairItem};
use mat::alg;
use mat::alg::SolveResult;
use mat::element::RefInv;
use mat::DataMatrix;
use mat::ConcatedMatrix;
use mat::Mat;
use std::rc::Rc;

use super::ExportType;
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
                        return Ok(List(Rc::new(ObjectPair {
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
                        return Ok(List(Rc::new(ObjectPair {
                            first: Lit(Matrix(MatrixWrap::Rat(Rc::new(general)))),
                            second: Lit(Matrix(MatrixWrap::Rat(Rc::new(special)))),
                        })));
                    }
                }
            }
            (a, b) => {
                return Err(EvalError::syntax(format!(
                    "Need two matrixes to solve, found {} and {}",
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

pub fn transposed(args: ObjectPairItem, _: &mut Environment) -> Output {
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
                return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(
                    DataMatrix::identity(r.0.try_into().unwrap()),
                )))));
            }
        }
        _ => return Err(EvalError::typ(format!("Can only transpose a matrix"))),
    }
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
                    for o in t.data.into_iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Flt(m))) => mt_data.push(m),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or float)")))
                        }
                    }
                    let concated = ConcatedMatrix::new(
                        mt_data.iter().map(|x| x.as_ref()).collect(),
                        t.rows,
                        t.cols
                    ).map_err(|e| EvalError::value(format!("{e}")))?;
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(concated)))))
                },
                Lit(Matrix(MatrixWrap::Rat(_))) => {
                    let mut mt_data = Vec::with_capacity(t.data.len());
                    for o in t.data.into_iter() {
                        match o {
                            Lit(Matrix(MatrixWrap::Flt(m))) => mt_data.push(m),
                            _ => return Err(EvalError::typ(format!("Can only concat matrix with same type of matrix (rational or float)")))
                        }
                    }
                    let concated = ConcatedMatrix::new(
                        mt_data.iter().map(|x| x.as_ref()).collect(),
                        t.rows,
                        t.cols
                    ).map_err(|e| EvalError::value(format!("{e}")))?;
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(concated)))))
                },
                _ => return Err(EvalError::typ(format!("Can only concat matrixes")))
            }
            
        },
        _ => return Err(EvalError::typ(format!("Can only concat a table of matrixes")))
    }
}


pub const EXPORTS: [ExportType; 10  ] = [
    ("inv", 1, &inv),
    ("eliminate", 1, &eliminate),
    ("rank", 1, &rank),
    ("reduce", 1, &reduce),
    ("det", 1, &det),
    ("solve", 2, &solve),
    ("transposed", 1, &transposed),
    ("trace", 1, &trace),
    ("nullspace", 1, &null_space),
    ("ridentity", 1, &ridentity),
];
