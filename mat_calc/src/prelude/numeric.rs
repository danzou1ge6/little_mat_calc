use indoc::indoc;
use mat::DataMatrix;
use mat::Mat;
use std::rc::Rc;

use crate::eval::BuiltinFunction;
use crate::eval::{Environment, ObjectPairItem};

use super::Output;
use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;
use crate::mat_wrap::MatrixWrap;

/// Numeric and logical operations

pub fn add(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Float(a + b))),
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a + b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(a.add(b.as_ref()))))));
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(a.add(b.as_ref()))))));
                }
                (Float(a), Matrix(MatrixWrap::Flt(b))) => {
                    let mut b = b.clone_data();
                    b.scale(a);
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(b)))));
                }
                (Rat(a), Matrix(MatrixWrap::Rat(b))) => {
                    let mut b = b.clone_data();
                    b.scale(a);
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(b)))));
                }
                (a, b) => return Err(EvalError::typ(format!("Can't add {} and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't add `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only add a Pair".to_string())),
    }
}

pub fn sub(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Float(a - b))),
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a - b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(a.sub(b.as_ref()))))));
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(a.sub(b.as_ref()))))));
                }
                (a, b) => {
                    return Err(EvalError::typ(format!(
                        "Can't subtract `{}` and `{}`",
                        a, b
                    )))
                }
            },
            (a, b) => {
                return Err(EvalError::typ(format!(
                    "Can't subtract `{}` and `{}`",
                    a, b
                )))
            }
        },
        _ => {
            return Err(EvalError::syntax(
                "You can only subtract a Pair".to_string(),
            ))
        }
    }
}

pub fn times(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Float(a * b))),
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a * b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    match a.dot(b.as_ref()) {
                        Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(r))))),
                        Err(e) => return Err(EvalError::value(format!("{e}"))),
                    }
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    match a.dot(b.as_ref()) {
                        Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Rat(Rc::new(r))))),
                        Err(e) => return Err(EvalError::value(format!("{e}"))),
                    }
                }
                (Rat(a), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Rat({
                        let mut r = b.clone_data();
                        r.scale(a);
                        Rc::new(r)
                    }))));
                }
                (Float(a), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Flt({
                        let mut r = b.clone_data();
                        r.scale(a);
                        Rc::new(r)
                    }))));
                }
                (a, b) => return Err(EvalError::typ(format!("Can't times `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't times `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only times a Pair".to_string())),
    }
}

pub fn devide(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Float(a / b))),
                (Rat(a), Rat(b)) => {
                    if b.0 == 0 {
                        return Err(EvalError::zero_division(format!("{}/0", a)));
                    }
                    return Ok(Lit(Rat(*a / b)));
                }
                (a, b) => return Err(EvalError::typ(format!("Can't devide `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't devide `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only devide a Pair".to_string())),
    }
}

pub fn eq(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Rat(a), Rat(b)) => return Ok(Lit(Bool(a == b))),
                (Nil, Nil) => return Ok(Lit(Bool(true))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Bool(a.as_ref() == b.as_ref())))
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Bool(a.as_ref() == b.as_ref())))
                }
                _ => return Ok(Lit(Bool(false))),
            },
            _ => return Ok(Lit(Bool(false))),
        },
        _ => return Err(EvalError::syntax("You can only compare a Pair".to_string())),
    }
}

pub fn lt(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Bool(a < b))),
                (Rat(a), Rat(b)) => return Ok(Lit(Bool(a < b))),
                (a, b) => return Err(EvalError::typ(format!("Can't compare `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't compare `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only compare a Pair".to_string())),
    }
}

pub fn gt(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => return Ok(Lit(Bool(a > b))),
                (Rat(a), Rat(b)) => return Ok(Lit(Bool(a > b))),
                (a, b) => return Err(EvalError::typ(format!("Can't compare `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't compare `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only compare a Pair".to_string())),
    }
}

pub fn or(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Bool(a), Bool(b)) => return Ok(Lit(Bool(*a || *b))),
                (a, b) => return Err(EvalError::typ(format!("Can't 'or' `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't 'or' `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only 'or' a Pair".to_string())),
    }
}

pub fn and(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Bool(a), Bool(b)) => return Ok(Lit(Bool(*a && *b))),
                (a, b) => return Err(EvalError::typ(format!("Can't 'and' `{}` and `{}`", a, b))),
            },
            (a, b) => return Err(EvalError::typ(format!("Can't 'and' `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only 'and' a Pair".to_string())),
    }
}

pub fn to_float(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        Lit(Rat(x)) => return Ok(Lit(Float(x.into()))),
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let m: DataMatrix<f64> = m.clone_data().convert();
            return Ok(Lit(Matrix(MatrixWrap::Flt(Rc::new(m)))));
        }
        other => {
            return Err(EvalError::typ(format!(
                "Can only convert rational or rational matrix to float, not {}",
                other
            )))
        }
    }
}

pub const EXPORTS: [BuiltinFunction; 10] = [
    BuiltinFunction {
        f: &add,
        name: "+",
        argn: 2,
        help: indoc! {"
            Usage: (+ a b) -> type(a)
            Add two numbers or two matrixes. They must have same kind of data type, rational or float.
            The latter matrix is truncated if it's larger, or is repeated by columns and rows if smaller.
            For example, [1 2 3; 4 5 6;] + [1 2; 4 5;] = [1+1 2+2 3+1; 4+4; 5+5, 6+4;] "},
    },
    BuiltinFunction {
        f: &sub,
        name: "-",
        argn: 2,
        help: indoc! {"
            Usage: (- a b) -> type(a)
            Sub `b` from `a`, `a` and `b` can be numbers or matrixes.
            If dimensions of matrixes are inconsistent, `b` is processed same way as `+`.  "},
    },
    BuiltinFunction {
        f: &times,
        name: "*",
        argn: 2,
        help: indoc! {"
            Usage: (* a b) -> type(b)
            Times `a` and `b` if they are numbers, and dot if they are matrixes.
            Or, `a` can be a number and `b` can be a matrix, then `b` is scaled by `a`, but not vice-versa.
            Colunms of `a` and rows of `b` must equal.  "},
    },
    BuiltinFunction {
        f: &devide,
        name: "/",
        argn: 2,
        help: indoc! {"
            Usage: (/ a b) -> type(b)
            Devide `a` by `b`. Both must be numbers.  "},
    },
    BuiltinFunction {
        f: &lt,
        name: "<",
        argn: 2,
        help: indoc! {"
            Usage: (< a b) -> bool
            Compare two numbers.  "},
    },
    BuiltinFunction {
        f: &gt,
        name: ">",
        argn: 2,
        help: indoc! {"
            Usage: (> a b) -> bool
            Compare two numbers.  "},
    },
    BuiltinFunction {
        f: &eq,
        name: "=",
        argn: 2,
        help: indoc! {"
            Usage: (= a b) -> bool
            Compare two numbers.  "},
    },
    BuiltinFunction {
        f: &and,
        name: "&",
        argn: 2,
        help: indoc! {"
            Usage: (& a: bool b: bool) -> bool
            AND operation.  "},
    },
    BuiltinFunction {
        f: &or,
        name: "|",
        argn: 2,
        help: indoc! {"
            Usage: (& a: bool b: bool) -> bool
            OR operation.  "},
    },
    BuiltinFunction {
        f: &to_float,
        name: "tofloat",
        argn: 1,
        help: indoc! {"
            Usage: (tofloat x: rational) -> float
                   (tofloat x: matrix<rational>) -> matrix<float>
            Convert rational to float.  "},
    },
];
