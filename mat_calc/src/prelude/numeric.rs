use crate::eval::{Environment, ObjectPairItem};

use super::ExportType;
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
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a + *b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(a.add(b.as_ref()))))));
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(a.add(b.as_ref()))))));
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
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a - *b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(a.sub(b.as_ref()))))));
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(a.sub(b.as_ref()))))));
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
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a * *b))),
                (Matrix(MatrixWrap::Flt(a)), Matrix(MatrixWrap::Flt(b))) => {
                    match a.dot(b.as_ref()) {
                        Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(r))))),
                        Err(e) => return Err(EvalError::value(format!("{e}"))),
                    }
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    match a.dot(b.as_ref()) {
                        Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(r))))),
                        Err(e) => return Err(EvalError::value(format!("{e}"))),
                    }
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
                (Rat(a), Rat(b)) => return Ok(Lit(Rat(*a / *b))),
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
                    return Ok(Lit(Bool(a.eq(b))))
                }
                (Matrix(MatrixWrap::Rat(a)), Matrix(MatrixWrap::Rat(b))) => {
                    return Ok(Lit(Bool(a.eq(b))))
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

pub fn log(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => match (&pair.first, &pair.second) {
            (Lit(a), Lit(b)) => match (a, b) {
                (Float(a), Float(b)) => {
                    if let Ok(a) = (*a).try_into() {
                        return Ok(Lit(Float(b.log(a))));
                    } else {
                        return Err(EvalError::value(format!(
                            "Exponent can't be negative: {}",
                            b
                        )));
                    }
                }
                (a, b) => {
                    return Err(EvalError::value(format!(
                        "Can't log `{}` and `{}`, must be floats",
                        a, b
                    )))
                }
            },
            (a, b) => return Err(EvalError::value(format!("Can't log `{}` and `{}`", a, b))),
        },
        _ => return Err(EvalError::syntax("You can only 'log' a Pair".to_string())),
    }
}

pub const EXPORTS: [ExportType; 10] = [
    ("+", 2, &add),
    ("-", 2, &sub),
    ("*", 2, &times),
    ("/", 2, &devide),
    ("<", 2, &lt),
    (">", 2, &gt),
    ("=", 2, &eq),
    ("&", 2, &and),
    ("|", 2, &or),
    ("log", 2, &log),
];
