use crate::eval::ObjectPair;
use crate::eval::{Environment, ObjectPairItem};

use super::{Output, ExportType};
use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;

use std::rc::Rc;

/// Get the first element of a scheme pair
pub fn car(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => return Ok(pair.first.clone()),
        _ => {
            return Err(EvalError::syntax(
                "Can only call `car` on a pair or a rational".to_string(),
            ))
        }
    }
}

/// Get the second element of a scheme pair
pub fn cdr(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(pair) => return Ok(pair.second.clone()),
        _ => {
            return Err(EvalError::syntax(
                "Can only call `cad` on a pair or a rational".to_string(),
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
                    List(_) => pair.second.clone(),
                    _ => List(Rc::new(ObjectPair {
                        first: pair.second.clone(),
                        second: Lit(Nil),
                    })),
                },
            };
            return Ok(List(Rc::new(list)));
        }
        _ => {
            return Err(EvalError::syntax(
                "Can only concat a pair into a list".to_string(),
            ))
        }
    }
}

pub const EXPORTS: [ExportType; 3] = [("car", 1, &car), ("cdr", 1, &cdr), ("con", 2, &con)];
