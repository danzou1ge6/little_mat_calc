use crate::eval::{EvalError, Frame, ObjectPairItem, BuiltinFunction};

use indoc::indoc;
use std::rc::Rc;

type Output = Result<ObjectPairItem, EvalError>;

mod misc;
mod constants;
mod list;
mod matrix;
mod numeric;

pub fn all_builtins() -> impl Iterator<Item=BuiltinFunction> {
    numeric::EXPORTS.into_iter()
        .chain(list::EXPORTS.into_iter())
        .chain(matrix::EXPORTS.into_iter())
        .chain(matrix::EXPORTS.into_iter())
        .chain(misc::EXPORTS.into_iter())
}

/// Inject all builtin functions in to `frame`, which is the root frame
pub fn inject_builtins(frame: &mut Frame) {
    for func in all_builtins() {
        frame.insert(
            func.name.to_string(),
            ObjectPairItem::BuiltinFunc(Rc::new(func)),
        );
    }

    for (val, name) in constants::EXPORTS.into_iter() {
        frame.insert(name.to_string(), ObjectPairItem::Lit(val));
    }
}

/// Defines some scheme functions
pub const PRELUDE_SRC: &'static str = indoc! {"
(
# Get the last item of a list
(def (last x)
    (if (= nil (cdr x))
        (car x)
        (last (cdr x))
    )
)

# abs
(def (abs x)
    (if (< x 0)
        (- 0 x)
        x
    )
)

# pow
(def (pow x n)
    (if (< n 2)
        x
        (* x (pow x (- n 1)))
    )
)
)"
};

pub fn get_prelude_src() -> String {
    PRELUDE_SRC
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect()
}


