use super::{environment::Environment, error::EvalError};
use crate::{mat_wrap::MatrixWrap, table::Table};
use mat::Rational;
use mat::Complex;
use std::rc::Rc;

#[derive(Clone)]
pub enum Literal {
    /// Rational, eg 3/7
    Rat(Rational),
    /// Complex, eg. `1.2 + 3j`
    Cplx(Complex),
    /// Matrix
    Matrix(MatrixWrap),
    ///
    Table(Box<Table<ObjectPairItem>>),
    /// `nil`
    Nil,
    Str(String),
    Bool(bool),
}

impl TryInto<Literal> for Token {
    type Error = ();

    /// Convert a [`Token`] literal into a [`Literal`]
    ///
    /// This might fail as [`Token`] can be a [`Token::Word`]
    fn try_into(self) -> Result<Literal, ()> {
        use Token::*;

        match self {
            Cplx(fl) => Ok(Literal::Cplx(fl)),
            Nil => Ok(Literal::Nil),
            Rat(r) => Ok(Literal::Rat(r)),
            Matrix(m) => Ok(Literal::Matrix(m)),
            Str(s) => Ok(Literal::Str(s)),
            Bool(b) => Ok(Literal::Bool(b)),
            _ => Err(()),
        }
    }
}

use crate::token_pair::*;

/// Represents a function defined in scheme
pub struct Function {
    /// The list of argument names
    pub args: TokenPairItem,
    /// The source of the function
    pub body: TokenPairItem,
    pub name: String,
}

/// Represents a builtin-function, eg add, substract, con
pub struct BuiltinFunction {
    pub f: &'static dyn Fn(ObjectPairItem, &mut Environment) -> Result<ObjectPairItem, EvalError>,
    pub argn: usize,
    pub name: &'static str,
    pub help: &'static str,
}

#[derive(Clone)]
/// Represents an item of a scheme pair, but it's evaluated `Object` instead of the source
pub enum ObjectPairItem {
    /// Literal value, eg 1, 'a', #f, nil
    Lit(Literal),
    /// Pair, which is usually used for lists, eg (1 (2 (3 nil)))
    List(Box<ObjectPair>),
    Func(Rc<Function>),
    BuiltinFunc(Rc<BuiltinFunction>),
}

#[derive(Clone)]
/// Represents a scheme pair which is pair
pub struct ObjectPair {
    pub first: ObjectPairItem,
    pub second: ObjectPairItem,
}

impl ObjectPairItem {
    pub fn make_list(mut v: Vec<ObjectPairItem>) -> Self {
        if v.len() == 1 {
            return ObjectPairItem::List(Box::new(ObjectPair {
                first: v.pop().unwrap(),
                second: ObjectPairItem::Lit(Literal::Nil),
            }));
        }
        let item = v.pop().unwrap();
        return ObjectPairItem::List(Box::new(ObjectPair {
            first: item,
            second: ObjectPairItem::make_list(v),
        }));
    }
}

mod display {
    use super::*;
    use std::fmt::Display;

    impl Display for Literal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Literal::*;

            match self {
                Rat(r) => r.fmt(f),
                Cplx(fl) => fl.fmt(f),
                Nil => write!(f, "nil"),
                Matrix(m) => write!(f, "\n{m}"),
                Table(t) => write!(f, "{t}"),
                Str(s) => {
                    if s.contains('\n') {
                        write!(f, "\n{s}")
                    } else {
                        write!(f, "{s}")
                    }
                }
                Bool(b) => {
                    if *b {
                        write!(f, "#t")
                    } else {
                        write!(f, "#f")
                    }
                }
            }
        }
    }

    impl Display for Function {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<function at {:?}>", self as *const Self)
        }
    }

    impl Display for BuiltinFunction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<bultin-function at {:?}", self as *const Self)
        }
    }

    impl Display for ObjectPairItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use ObjectPairItem::*;

            match self {
                Lit(l) => l.fmt(f),
                List(p) => p.fmt(f),
                Func(func) => func.fmt(f),
                BuiltinFunc(func) => func.fmt(f),
            }
        }
    }
    impl std::fmt::Debug for ObjectPairItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (self as &dyn Display).fmt(f)
        }
    }

    impl Display for ObjectPair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} {})", self.first, self.second)
        }
    }
}
