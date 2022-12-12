use crate::eval::{BuiltinFunction, EvalError, Frame, ObjectPairItem};

use indoc::indoc;
use std::rc::Rc;

type Output = Result<ObjectPairItem, EvalError>;

mod list;
mod matrix;
mod misc;
mod numeric;

pub fn all_builtins() -> impl Iterator<Item = BuiltinFunction> {
    numeric::EXPORTS
        .into_iter()
        .chain(list::EXPORTS.into_iter())
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
}

/// Defines some scheme functions
pub const PRELUDE_SRC: &'static str = indoc! {"
(
(def _help_last \"Get the last item of a list, usually for evaluating multiple
expression and returning the last one in a function
\")
(def (last x)
    (if (= nil (cdr x))
        (car x)
        (last (cdr x))
    )
)

# abs
(def _help_abs \"Get the absolute value of a rational\")
(def (abs x)
    (if (< x 0)
        (- 0 x)
        x
    )
)

# pow
(def _help_pow \"Usage: (pow x n) Calculate power of `x` by `n`\")
(def (pow x n)
    (if (< n 2)
        x
        (* x (pow x (- n 1)))
    )
)

# adjoint
(def _help_adjoint \"Calculate the adjoint of a complex\")
(def (adjoint x)
    (+
        (car x)
        (* -1j (cdr x))
    )
)

(def _help_normal2 \"Calculate the normal square of a complex\")
(def (normal2 x)
    (* x (adjoint x))
)

(def _help_madjoint \"Calculate the adjoint matrix of a matirx, if invertable\")
(def (madjoint x)
    (* (det x) (inv x))
)

(def _help_eigvecof \"Usage: (eigvecof x: matrix v: complex)
Get the eigen vector of `x` corresponding to eigen value `v`
\")
(def (eigvecof x v)
    (nspace (- x (* v ci (car dim x))))
)

(def _help_geigvecof \"Usage: (geigvecof x: matrix v: complex)
Get the generalized eigen vector of `x` corresponding to eigen value `v`\")
(def (geigvecof x v)
    (nspace
        (pow
            (- x (* v ci (car dim x)))
            (car dim x)
        )
    )
)

# eigen values and eigen vectors
(def _help_eigvec \"Get the eigen vectors of a matrix, repeated ones aren't deleted\")
(def (eigvec x) (last
    (def eigvals (eigval x))
    (def (_eigvec x vals n)
        (if (< n (- (car (dim vals)) 1))
            (last
                (def later (_eigvec x vals (+ n 1)))
                (def this-eigval (get vals (n 0)))
                (def this (nspace (- x (* this-eigval (ci (car (dim x)))))))
                (concat [later this;])
                nil
            )
            (last
                (def this-eigval (get vals (n 0)))
                (nspace (- x (* this-eigval (ci (car (dim x))))))
                nil
            )
        )
    )
    (_eigvec x eigvals 0)
    nil
))

(def _help_row \"Get one row of a matrix\")
(def (row m i) (slice m i 1 0 (cdr dim m)))

(def _help_col \"Get one column of a matrix\")
(def (col m i) (slice m 0 (car dim m) i 1))

(def preludes \"last pow eigvec abs adjoint normal2 madjoint eigvecof geigvecof row col\")
)"
};

pub fn get_prelude_src() -> String {
    let filtered_lines: Vec<&str> = PRELUDE_SRC
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect();
    filtered_lines.join("\n")
}
