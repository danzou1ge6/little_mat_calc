use super::{Output, all_builtins};
use indoc::indoc;
use mat::Rational;

use crate::eval::{Environment, ObjectPairItem, BuiltinFunction};

use crate::eval::EvalError;
use crate::eval::Literal::*;
use crate::eval::ObjectPairItem::*;

const GENERAL_HELP: &str = indoc! {"
LITERALS
    A variety of literals are supported, including
    - numbers, which are further categories into
        - rational (integers are included), for example `1/3`, `2`
        - float, for example `0.2`
    - matrixes, which can consist of rational elements or float elements
      matrixes are represented by brackets, for example
        `[1 2; 3 4;]`
      `;` is used to sperate rows
    - bool, represented by `#t`(true) and `#f`(false)
    - symbol table, which is represented in the same way as matrixes do, but
      only consists of variable names, for example,
        `[a b; c d;]`

DEFINING VARIABLES
    `def` keyword can be used to define variables, for example
        `(def x 1)`

IF BRANCH
    `if` can be used to cotrol execution, syntax of which is
        `(if <test> <then> <else>)`
    where `test` is a bool expression, if `test` is true, `then` is evaluated,
    otherwise `else` is evaluated and returned

DEFINING FUNCTIONS
    A kind of simple function is supported, they can be defined using `def`
    keyword, for example,
        `(def (pow x n) (if (< n 2) x (* x (pow x (- n 1)))))`
    defines a function that calculate `x * x * ... * x` `n` times

BUILTINS
    There are a range of builtin-functions, for example
        `(inv m)`
    tries to calculate and return the inversion of `m`.
    For any given builtin, use
        `(help <name>)`
    to get detailed information.
    To get a list of available builtins, use
        `(help 1)`
"};


pub fn help(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        BuiltinFunc(f) => {
            return Ok(Lit(Str(f.help.to_string())));
        },
        Lit(Rat(Rational(0, 1))) => {
            return Ok(Lit(Str(GENERAL_HELP.to_string())));
        },
        Lit(Rat(Rational(1, 1))) => {
            let names: Vec<&str> = all_builtins().map(|b| b.name).collect();
            return Ok(Lit(Str(
                format!("{}\nUse `(help <name>)` to get detailed information of builtin `<name>`", names.join(" "))
            )));
        }
        _ => return Err(EvalError::typ(format!("Can only call `help` on builtins; Or use`(help 0)` for general help")))
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

