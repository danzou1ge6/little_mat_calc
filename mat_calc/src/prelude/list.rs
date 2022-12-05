use indoc::indoc;
use mat::{Rational};

use crate::eval::ObjectPair;
use crate::eval::{Environment, ObjectPairItem};

use super::Output;
use crate::eval::BuiltinFunction;
use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;

/// Get the first element of a scheme pair
pub fn car(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => return Ok(pair.first.clone()),
        Lit(Rat(r)) => return Ok(Lit(Rat(Rational(r.0, 1)))),
        Lit(Cplx(c)) => return Ok(Lit(Cplx(c.re().into()))),
        _ => {
            return Err(EvalError::syntax(
                "Can only call `car` on a pair or a rational or a complex".to_string(),
            ))
        }
    }
}

/// Get the second element of a scheme pair
pub fn cdr(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => return Ok(pair.second.clone()),
        Lit(Rat(r)) => return Ok(Lit(Rat(Rational(r.1, 1)))),
        Lit(Cplx(c)) => return Ok(Lit(Cplx(c.im().into()))),
        _ => {
            return Err(EvalError::syntax(
                "Can only call `cad` on a pair or a rational or a complex".to_string(),
            ))
        }
    }
}

/// Concat the two arguments to a list
///
/// eg. (con 1 2) = (1 (2 nil))
pub fn con(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => {
            let list = ObjectPair {
                first: pair.first.clone(),
                second: match pair.second {
                    List(_) => pair.second,
                    _ => List(Box::new(ObjectPair {
                        first: pair.second,
                        second: Lit(Nil),
                    })),
                },
            };
            return Ok(List(Box::new(list)));
        }
        _ => {
            return Err(EvalError::syntax(
                "Can only concat a pair into a list".to_string(),
            ))
        }
    }
}

pub const EXPORTS: [BuiltinFunction; 3] = [
    BuiltinFunction {
        f: &car,
        argn: 1,
        name: "car",
        help: indoc! {"
            Get
            - the first element of a pair-
            - or the real part of a complex
            - or the numerator of a rational
            The return is same as `cdr` "},
    },
    BuiltinFunction {
        f: &cdr,
        argn: 1,
        name: "cdr",
        help: indoc! {"
            Get
            - the second element of a pair-
            - or the imagine part of a complex (returns a complex with 0 imagine)
            - or the dominator of a rational (returns a rational with 1 dominator)"},
    },
    BuiltinFunction {
        f: &con,
        argn: 2,
        name: "con",
        help: "Concat two elements to a list",
    },
];
