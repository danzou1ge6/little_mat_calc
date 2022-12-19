#![feature(box_patterns)]
mod splitting;

pub mod eval;
pub mod interpreter;
mod mat_wrap;
mod prelude;
mod table;
mod token_pair;

use mat_macro::{compiler_host, compiler_version};
use indoc::indoc;

pub fn startup_text() -> String {
    format!(
        indoc !{"
        Little Mat Calculator {} [rustc {}] on {}
        Type \".help\" to get more information"},
        env!("CARGO_PKG_VERSION"),
        compiler_version!(),
        compiler_host!(),
    )
}

pub const STANDBY_PROMPT: &str = "> ";
pub const PENDING_PROMPT: &str = ". ";
pub const ERROR_PROMPT: &str = "! ";
pub const RESULT_PROMPT: &str = "= ";
