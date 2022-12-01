use crate::eval::{Environment, ObjectPairItem};
use mat::Mat;
use mat::alg;
use mat::element::RefInv;

use super::ExportType;
use super::Output;
use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;
use crate::mat_wrap::MatrixWrap;


pub fn inv(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(_) => return Err(EvalError::syntax("You can only inv one item a time".to_string())),
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            match alg::inv(&mut m.clone_data()) {
                Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(r))))),
                Err(e) => return Err(EvalError::value(format!("{e}"))),
            }
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            match alg::inv(&mut m.clone_data()) {
                Ok(r) => return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(r))))),
                Err(e) => return Err(EvalError::value(format!("{e}"))),
            }
        },
        Lit(Rat(r)) => return Ok(Lit(Rat(r.inv()))),
        Lit(Float(f)) => return Ok(Lit(Float(f.inv()))),
        other => return Err(EvalError::typ(format!("Can't inv {other}")))
    }
}

pub fn eliminate(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(_) => return Err(EvalError::syntax("You can only gussain eliminate one item a time".to_string())),
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let r = m.clone_data().eliminated();
            return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(r)))));
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated();
            return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(r)))));
        }
        _ => return Err(EvalError::typ(format!("Can only eliminate a matrix")))
    }
}


pub fn reduce(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(_) => return Err(EvalError::syntax("You can only gussian eliminate an item a time".to_string())),
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let r = m.clone_data().eliminated().reduced();
            return Ok(Lit(Matrix(MatrixWrap::Flt(Box::new(r)))));
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated().reduced();
            return Ok(Lit(Matrix(MatrixWrap::Rat(Box::new(r)))));
        }
        _ => return Err(EvalError::typ(format!("Can only eliminate a matrix")))
    }
}

pub fn rank(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        List(_) => return Err(EvalError::syntax("You can only calculate rank one item a time".to_string())),
        Lit(Matrix(MatrixWrap::Flt(m))) => {
            let r = m.clone_data().eliminated().rank() as i32;
            return Ok(Lit(Rat(r.into())));
        },
        Lit(Matrix(MatrixWrap::Rat(m))) => {
            let r = m.clone_data().eliminated().rank() as i32;
            return Ok(Lit(Rat(r.into())));
        },
        _ => return Err(EvalError::typ(format!("Can only calculate rank of a matrix")))
    }
}

// pub fn solve(args: ObjectPairItem, _: &mut Environment) -> Output {
//     match args {
//         List(pair) => match (&pair.first, &pair.second) {
//             (Lit(Matrix(MatrixWrap::Flt(a))), Lit(Matrix(MatrixWrap::Flt(b)))) => {
//                 let r = alg::solve(a, b);

//             },
//             (Lit(Matrix(MatrixWrap::Rat(a))), Lit(Matrix(MatrixWrap::Rat(b)))) => {

//             }
//         }
//         _ => return Err(EvalError::syntax("Need two arguments to solve linear equation".to_string())),
//     }

// }


pub const EXPORTS: [ExportType; 4] = [
    ("inv", 1, &inv), ("eliminate", 1, &eliminate), ("rank", 1, &rank),
    ("reduce", 1, &reduce)
];
