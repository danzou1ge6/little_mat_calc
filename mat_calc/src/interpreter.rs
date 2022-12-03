use super::eval::Config;
use super::eval::{Environment, EvalError, ObjectPairItem};
use super::prelude::{get_prelude_src, inject_builtins};
use super::splitting::SplitBuffer;
pub use super::token_pair::PendingResult;
use super::token_pair::{ParseError, TokenPairParser};

mod error {
    use super::EvalError;
    use super::ParseError;

    pub enum InterpreterError {
        Eval(EvalError),
        Parse(ParseError),
    }

    impl Into<InterpreterError> for EvalError {
        fn into(self) -> InterpreterError {
            InterpreterError::Eval(self)
        }
    }
    impl Into<InterpreterError> for ParseError {
        fn into(self) -> InterpreterError {
            InterpreterError::Parse(self)
        }
    }

    impl std::fmt::Display for InterpreterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                InterpreterError::Eval(err) => err.fmt(f),
                InterpreterError::Parse(err) => err.fmt(f),
            }
        }
    }
}

use self::error::InterpreterError;

pub struct Interpreter {
    parser: TokenPairParser,
    pub env: Environment,
}

impl Interpreter {
    pub fn new(config: Config) -> Self {
        let mut env = Environment::new(config);
        inject_builtins(env.root_frame());

        let mut n = Self {
            parser: TokenPairParser::new(),
            env,
        };

        n.eval_line(&get_prelude_src());

        return n;
    }

    pub fn eval_line(&mut self, src: &str) -> PendingResult<ObjectPairItem, InterpreterError> {
        let split_buffer = SplitBuffer::new(&src);
        let pieces: Vec<&str> = split_buffer.collect();
        match self.parser.parse(pieces) {
            PendingResult::Pending => return PendingResult::Pending,
            PendingResult::Ok(tokens) => {
                self.parser.clear();
                match self.env.eval(tokens) {
                    Ok(obj) => return PendingResult::Ok(obj),
                    Err(err) => return PendingResult::Err(err.into()),
                }
            }
            PendingResult::Err(err) => {
                self.parser.clear();
                return PendingResult::Err(err.into());
            }
        }
    }
}
