mod object;
mod frame;
mod environment;
mod error;
mod config;

pub use object::{BuiltinFunction, ObjectPair, ObjectPairItem, Literal};
pub use environment::Environment;
pub use error::EvalError;
pub use frame::Frame;
pub use config::Config;
