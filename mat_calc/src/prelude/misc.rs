use super::{all_builtins, Output};
use indoc::indoc;
use mat::Rational;

use crate::eval::{BuiltinFunction, Environment, ObjectPairItem};

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
    `def` keyword can be used to define variables
        `(def <variable-name> <value>)`
    For example
        `(def x 1)`

BUILTINS
    There are a range of builtin-functions, for example
        `(inv m)`
    tries to calculate and return the inversion of `m`.
    For any given builtin, use
        `(help <builtin-name>)`
    to get detailed information.
    To get a list of available builtins, use
        `(help 1)`

CALLING FUNCTIONS
    Functions can be called using the syntax:
        `(<function-name> <arguments>... )`
    For example, to get the (`i`, `j`) element of a matrix `m`, use
        `(get m i j)`

IF BRANCH
    `if` can be used to cotrol execution, syntax of which is
        `(if <test> <then> <else>)`
    where `test` is a bool expression, if `test` is true, `then` is evaluated,
    otherwise `else` is evaluated and returned

DEFINING FUNCTIONS
    A kind of simple function is supported, they can be defined using `def`,
        `(def (<function-name> <argument-names>...) <function-body>)`
    When executed, arguments are binded to `<argument-names>` and then
    `<function-body>` is evaluated, which is the result of the function execution.

    For example,
        `(def (pow x n) (if (< n 2) x (* x (pow x (- n 1)))))`
    defines a function that calculate `x * x * ... * x` `n` times

"};

pub fn help(args: ObjectPairItem, _: &mut Environment) -> Output {
    match args {
        BuiltinFunc(f) => {
            return Ok(Lit(Str(f.help.to_string())));
        }
        Lit(Rat(Rational(0, 1))) => {
            return Ok(Lit(Str(GENERAL_HELP.to_string())));
        }
        Lit(Rat(Rational(1, 1))) => {
            let names: Vec<&str> = all_builtins().map(|b| b.name).collect();
            return Ok(Lit(Str(format!(
                "{}\nUse `(help <name>)` to get detailed information of builtin `<name>`",
                names.join(" ")
            ))));
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only call `help` on builtins; Or use`(help 0)` for general help"
            )))
        }
    }
}

pub fn maxrecur(args: ObjectPairItem, env: &mut Environment) -> Output {
    match args {
        Lit(Rat(r)) => {
            if r.1 != 1 {
                return Err(EvalError::value(format!(
                    "Can only set max recursion to integer"
                )));
            }
            if r.0 == 0 {
                let x: i32 = env.config.max_recursion.try_into().map_err(|_| {
                    EvalError::value(format!(
                        "Current recursion limit is too large to represent in i32"
                    ))
                })?;
                return Ok(Lit(Rat(x.into())));
            }
            env.config.max_recursion =
                r.0.try_into()
                    .map_err(|_| EvalError::value(format!("Can't cast to usize")))?;
            return Ok(Lit(Nil));
        }
        _ => {
            return Err(EvalError::typ(format!(
                "Can only set max recursion to positive integer"
            )));
        }
    }
}

pub fn trace_back(args: ObjectPairItem, env: &mut Environment) -> Output {
    match args {
        Lit(Bool(b)) => {
            env.config.trace_back = b;
            Ok(Lit(Nil))
        }
        _ => Err(EvalError::typ(format!(
            "Can only set trace back on/off to bool"
        ))),
    }
}

pub const EXPORTS: [BuiltinFunction; 3] = [
    BuiltinFunction {
        f: &help,
        name: "help",
        argn: 1,
        help: "Display help information on builtins.",
    },
    BuiltinFunction {
        f: &maxrecur,
        name: "maxrecur",
        argn: 1,
        help: indoc! {"
            Usage: (maxrecur n: rational) -> nil | rational
            Set the max recursion limit to `n`, where `n` must be positive integer.
            Or, if `n` is `0`, returns current limit.
            If `n` is too large, stack overflow might occur.
        "},
    },
    BuiltinFunction {
        f: &trace_back,
        name: "traceback",
        argn: 1,
        help: indoc! {"
            Usage: (traceback on: bool) -> nil
            Toggle if trace back is on
        "},
    },
];
