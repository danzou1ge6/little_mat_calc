use indoc::indoc;
use mat_calc::eval::Config;
use mat_calc::interpreter::Interpreter;
use mat_calc::interpreter::PendingResult;

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process::exit;

use mat_macro::{compiler_host, compiler_version};

fn startup_text() -> String {
    format!(
        "Little Mat Calculator {} [rustc {}] on {}\nType \".help\" to get more information",
        env!("CARGO_PKG_VERSION"),
        compiler_version!(),
        compiler_host!()
    )
}

const STANDBY_PROMPT: &str = "> ";
const PENDING_PROMPT: &str = ". ";
const ERROR_PROMPT: &str = "! ";
const RESULT_PROMPT: &str = "=> ";

const HELP_TEXT: &str = indoc! {"
AVAILABLE INTEREPTER COMMANDS
    - `.quit` exits the interepter
    - `.evalf <path>` evaluates the file at <path>
    - `.help` displays this message

HELP ON THE CALCULATOR SYNTAX
    The syntax of this calculator is inspired by scheme.
    Everything in scheme are *expressions*, and are surrounded by parentheses,
    for example,
        `(1)`
    is a valid expression and evaluates to the rational 1.

    For more information, type
        `(help 0)`
"};

/// Lines starting with `#` are regarded as annotations
fn strip_anno_lines(src: String) -> String {
    src.lines().filter(|line| !line.starts_with('#')).collect()
}

fn evalf(path: &str, intp: &mut Interpreter) -> Result<(), String> {
    let mut file = File::open(path)
        .map_err(|e| format!("Can't open file: {e}"))?;

    let mut src = Vec::new();
    file.read_to_end(&mut src)
        .map_err(|e| format!("Can't read file: {e}"))?;
    let src = String::from_utf8(src)
        .map_err(|e| format!("Not valid utf-8 file: {}", e))?;

    let src = strip_anno_lines(src);
    interpreter_eval_and_prin(intp, &src);

    Ok(())
}

/// Run a .x command
fn command(cmd: &str, intp: &mut Interpreter) -> Result<(), String> {
    if cmd == ".quit" {
        exit(0);

    } else if cmd.starts_with(".evalf ") {
        let path = cmd.trim_start_matches(".evalf ");
        return evalf(path, intp);

    } else if cmd == ".help" {
        println!("{HELP_TEXT}");
        return Ok(());

    } else {
        return Err("No such command. Type .help for help".to_string());
    }
}

fn interpreter_eval_and_prin<'a>(intp: &'a mut Interpreter, src: &'a str) {
    match intp.eval_line(src) {
        PendingResult::Pending => {
            print!("{}", PENDING_PROMPT);
        }
        PendingResult::Ok(obj) => {
            println!("{}{}", RESULT_PROMPT, obj);
            print!("{}", STANDBY_PROMPT);
        }
        PendingResult::Err(err) => {
            println!("{}{}", ERROR_PROMPT, err);
            print!("{}", STANDBY_PROMPT);
        }
    }
}

fn interpreter_loop(intp: &mut Interpreter) {
    print!("> ");

    loop {
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.starts_with('.') {
            match command(line, intp) {
                Ok(_) => print!("{STANDBY_PROMPT}"),
                Err(s) => print!("{}{}\n{}", ERROR_PROMPT, s, STANDBY_PROMPT),
            }
        } else {
            interpreter_eval_and_prin(intp, line);
        }

    }
}

fn app_main() {
    let config = Config {
        trace_back: false,
        max_recursion: 1000,
    };
    let mut intp = Interpreter::new(config);

    println!("{}", startup_text());
    interpreter_loop(&mut intp);
}

fn main() {
    use std::thread::Builder;
    let handler = Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(app_main)
        .unwrap();
    handler.join().unwrap();
}
