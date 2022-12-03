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
        "Little Mat Calculator [rustc {}] on {}\nType .help to get help",
        compiler_version!(),
        compiler_host!()
    )
}

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

/// Run a .x command
fn command(cmd: &str, intp: &mut Interpreter) {
    if cmd == ".quit" {
        exit(0);
    } else if cmd.starts_with(".evalf ") {
        match File::open(cmd.trim_start_matches(".evalf ")) {
            Ok(mut file) => {
                let mut src = Vec::new();
                match file.read_to_end(&mut src) {
                    Ok(_) => {
                        if let Ok(src) = String::from_utf8(src) {
                            interpreter_eval_and_prin(intp, &strip_anno_lines(src));
                        } else {
                            println!("File is not valid utf-8 encoded source code");
                        }
                    }
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("{e}"),
        }
    } else if cmd == ".help" {
        println!("{HELP_TEXT}");
        print!("> ");
    } else {
        println!("No such command. Type .help for help");
        print!("> ");
    }
}

fn interpreter_eval_and_prin<'a>(intp: &'a mut Interpreter, src: &'a str) {
    match intp.eval_line(src) {
        PendingResult::Pending => {
            print!(". ");
        }
        PendingResult::Ok(obj) => {
            println!("=> {}", obj);
            print!("> ");
        }
        PendingResult::Err(err) => {
            println!("! {}", err);
            print!("> ");
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
            command(line, intp);
            continue;
        }

        interpreter_eval_and_prin(intp, line);
    }
}
fn main() {
    let config = Config {
        trace_back: false,
        max_recursion: 1000,
    };
    let mut intp = Interpreter::new(config);

    println!("{}", startup_text());
    interpreter_loop(&mut intp);
}
