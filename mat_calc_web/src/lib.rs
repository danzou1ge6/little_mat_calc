use std::ops::{DerefMut, Deref};

use mat_calc::eval::Config;
use mat_calc::interpreter::{Interpreter, PendingResult};
use mat_calc::{ERROR_PROMPT, PENDING_PROMPT, RESULT_PROMPT, STANDBY_PROMPT};

use send_wrapper::SendWrapper;
use wasm_bindgen::prelude::*;
static mut INTEREPTER: Option<SendWrapper<Interpreter>> = None;

fn _init() {
    let config = Config {
        trace_back: false,
        max_recursion: 1000,
    };
    let intp = Interpreter::new(config);

    unsafe {
        INTEREPTER = Some(SendWrapper::new(intp));
    }
}

use mat_macro::{compiler_host, compiler_version};

pub fn _startup_text() -> String {
    format!(
        "Little Mat Calculator {} [rustc {}] on {}\nType \"(help 0)\" to get more information",
        env!("CARGO_PKG_VERSION"),
        compiler_version!(),
        compiler_host!()
    )
}

use std::panic::set_hook;

#[wasm_bindgen]
pub fn set_panic_hook(f: js_sys::Function) {
    let f = SendWrapper::new(f);

    set_hook(Box::new(move |info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            format!("{s:?}")
        } else {
            format!("A panic occured")
        };
        f.deref().call1(&JsValue::undefined(), &JsValue::from_str(&msg)).unwrap();
    }));
}


#[wasm_bindgen]
pub fn intp_init() {
    _init()
}

#[wasm_bindgen]
pub fn standby_prompt() -> String { STANDBY_PROMPT.to_string() }

#[wasm_bindgen]
pub fn startup_text() -> String {
    _startup_text()
}

#[wasm_bindgen]
pub struct EvalResult {
    output: String,
    prompt: String,
}

#[wasm_bindgen]
impl EvalResult {
    pub fn output(&self) -> String { self.output.clone() }
    pub fn prompt(&self) -> String { self.prompt.clone() }
}

fn interpreter_eval_and_print(intp: &mut Interpreter, src: &str) -> EvalResult {
    match intp.eval_line(src) {
        PendingResult::Ok(obj) => EvalResult {
            output: format!("{}{}", RESULT_PROMPT, obj),
            prompt: STANDBY_PROMPT.to_string(),
        },
        PendingResult::Err(err) => EvalResult {
            output: format!("{}{}", ERROR_PROMPT, err),
            prompt: STANDBY_PROMPT.to_string(),
        },
        PendingResult::Pending => EvalResult {
            output: String::new(),
            prompt: PENDING_PROMPT.to_string(),
        },
    }
}

#[wasm_bindgen]
pub fn intp_eval(src: String) -> EvalResult {
    unsafe {
        if INTEREPTER.is_none() {
            _init();
        }
        if let Some(sw_intp) = &mut INTEREPTER {
            let intp = sw_intp.deref_mut();
            interpreter_eval_and_print(intp, &src)
        } else {
            panic!("Interpreter not initialized");
        }
    }
}
