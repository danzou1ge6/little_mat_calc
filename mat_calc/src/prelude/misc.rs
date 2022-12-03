use super::Output;

use crate::eval::{Environment, ObjectPairItem, BuiltinFunction};

use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;


pub fn help(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        BuiltinFunc(f) => {
            return Ok(Lit(Str(f.help.to_string())));
        },
        _ => return Err(EvalError::typ(format!("Can only call `help` on builtins")))
    }
}

pub const EXPORTS: [BuiltinFunction; 1] = [
    BuiltinFunction {
        f: &help,
        name: "help",
        argn: 1,
        help: "Display help information on builtins."
    }
];

