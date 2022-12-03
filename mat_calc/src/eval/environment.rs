use super::frame::*;
use super::object::*;
use super::error::*;
use crate::table;
use crate::token_pair::Token;
use crate::token_pair::{TokenPairItem, TokenPair};
use super::config::Config;

use std::rc::Rc;

/// The environment where the evaluation happens
/// 
/// Manages a stack of [`Frame`], which are the namespace of each function call
pub struct Environment {
    frames: Vec<Frame>,
    pub config: Config
}

impl Environment {
    pub fn new(cfg: Config) -> Self {
        Self {
            frames: vec![Frame::new()],
            config: cfg
        }
    }

    /// Get the top frame in the stack
    pub fn frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    /// Get the root frame where builtins are defined
    pub fn root_frame(&mut self) -> &mut Frame {
        self.frames.first_mut().unwrap()
    }

    /// push a new frame for function call
    fn push_frame(&mut self) {
        self.frames.push(Frame::new())
    }

    /// Pop the upper frame after function call
    fn pop_frame(&mut self) {
        self.frames.pop();
    }

    /// Find an object with a name in the frame stack
    /// 
    /// Frames are searched from top to bottom
    fn find_object(&self, name: &str) -> Option<ObjectPairItem> {
        for frame in self.frames.iter().rev() {
            if let Some(obj) = frame.get(name) { return Some(obj) };
        }
        return None;
    }

    /// Define a object with a name in the top frame
    fn define_var(&mut self, name: String, value: ObjectPairItem) {
        self.frame().insert(name, value);
    }

    /// Define a function with a name in the top frame
    fn define_func(
        &mut self, name: String, param_names: TokenPairItem, body: TokenPairItem
    ) {
        let obj = ObjectPairItem::Func(Rc::new(Function {
            args: param_names,
            body,
            name: name.clone()
        }));
        self.frame().insert(name, obj);
    }

    /// Bind evaluated values of `values` to `names` in the top frame
    /// 
    /// After this operation, evaluating the `body` of [`Function`], which is the source code
    /// of the function, equals to calling the function
    fn bind_var(&mut self, names: TokenPairItem, values: TokenPairItem) -> Result<(), EvalError> {
        use TokenPairItem::*;
        use Token::*;

        match names {
            Pir(name_pair) => {
                if let Tok(Word(name)) = name_pair.first {
                    if let Pir(value_pair) = values {
                        let args = self.eval(value_pair.first.clone())?;
                        self.define_var(name, args);
                        return self.bind_var(name_pair.second, value_pair.second.clone())
                    } else {
                        return Err(EvalError::syntax(format!("Not enough parameters, expected `{}`", name)));
                    }
                } else {
                    return Err(EvalError::syntax("Parameter name list can only have Token::Word".to_string()))
                }
            },
            Tok(Word(name)) => {
                let args = self.eval(values)?;
                self.define_var(name, args);
                return Ok(());
            },
            Tok(_) => {
                return Err(EvalError::syntax("Parameter name list can only have Token::Word".to_string()));
            }
        }
    }

    /// Call a scheme-defined function
    /// 
    /// # It does the following
    /// - push a new frame to the stack
    /// - bind the parameters to the name of arguments of the function
    /// - eval the function body, which is just the source code of the function
    /// - pop the frame
    /// 
    /// The frame will be poped if an error happens
    fn apply_func(
        &mut self, func: &Function, parameters: TokenPairItem
    ) -> Result<ObjectPairItem, EvalError> {

        if self.frames.len() == self.config.max_recursion {
            return Err(EvalError::recursion(format!("Max recursion depth reached calling {}", func.name)));
        }

        self.push_frame();

        if let Err(e) = self.bind_var(func.args.clone(), parameters) {
            self.pop_frame();
            return Err(if self.config.trace_back {e.cat_msg(format!("calling `{}`", func.name))} else {e});
        }

        let result = self.eval(func.body.clone())
            .map_err(|e| 
                if self.config.trace_back {
                    e.cat_msg(
                        format!("calling `{}` with `{}`", func.name, self.frame().format_func_args())
                    )
                }
                else {e}
            );
        self.pop_frame();

        return result;
    } 

    /// For calling builtin-functions
    /// 
    /// Since only the number of arguments of a builtin-function is known, this function
    /// splits the `parameters` into `n` parts and evaluets them seperatly, then returning
    /// the evaluation result
    fn eval_n_times(&mut self, parameters: TokenPairItem, n: usize) -> Result<ObjectPairItem, EvalError> {
        use TokenPairItem::*;

        if n == 1 { return self.eval(parameters); }
        else {
            match parameters {
                Tok(_) => 
                    Err(EvalError::syntax(
                        format!("Not enough arguments for builtin function, {} needed", n)
                    )),
                Pir(pair) => {
                    let first = self.eval(pair.first)?;
                    let second = self.eval_n_times(pair.second, n - 1)?;

                    return Ok(ObjectPairItem::List(Box::new(ObjectPair {
                        first, second
                    })));
                }
            }
        }
    }

    /// Call a builtin-function
    /// 
    /// # It does
    /// - push a frame to the stack
    /// - call `eval_n_times` and get the arguments
    /// - call the rust function
    /// - pop the frame
    /// 
    /// The frame will be poped if an error happens
    fn apply_builtin_func(
        &mut self, func: &BuiltinFunction, parameters: TokenPairItem
    ) -> Result<ObjectPairItem, EvalError> {
        self.push_frame();
        let args = self.eval_n_times(parameters, func.argn)
            .map_err(|e| 
                {
                    self.pop_frame();
                    if self.config.trace_back { e.cat_msg(format!("calling builtin `{}`", func.name)) }
                    else { e }
                }
            )?;

        let result = (func.f)(args.clone(), self)
            .map_err(|e| 
                {
                    self.pop_frame();
                    if self.config.trace_back { e.cat_msg(format!("calling builtin `{}` with `{}`", func.name, args)) }
                    else {e}
                }
            )?;

        self.pop_frame();
        return Ok(result);
    }

    /// Call the `define` keyword
    fn define(&mut self, args: TokenPairItem) -> Result<(), EvalError> {
        use TokenPairItem::*;
        use Token::*;

        match args {
            Tok(_) => return Err(EvalError::syntax("Missing arguments for `def`".to_string())),
            Pir(pair) => {
                let sig = pair.first;
                let body = pair.second;

                match sig {
                    Tok(token) => {
                        match token {
                            Word(name) => {
                                let value = self.eval(body)?;
                                self.define_var(name, value);
                                return Ok(());
                            },
                            _ => return Err(EvalError::syntax("Need a Token::Word as variable name".to_string()))
                        }
                    },
                    Pir(pair) => {
                        match pair.first {
                            Tok(Word(name)) => {
                                self.define_func(name, pair.second, body);
                                return Ok(());
                            },
                            _ => return Err(EvalError::syntax("First item of function signature should be func name".to_string())),
                        }
                    }
                }
            }
        }
    }

    /// Call the `lambda` keyword
    fn lambda(&mut self, args: TokenPairItem) -> Result<Function, EvalError> {
        use TokenPairItem::*;

        match args {
            Tok(_) => return Err(EvalError::syntax("Missing arguments for `lambda`".to_string())),
            Pir(pair) => return Ok(Function {
                args: pair.first, body: pair.second, name: String::from("<lambda>")
            })
        }
    } 

    /// Call the `if` keyword
    fn _if(&mut self, args: TokenPairItem) -> Result<ObjectPairItem, EvalError> {
        use TokenPairItem::*;
        use ObjectPairItem::*;
        use Literal::*;

        match args {
            Pir(box TokenPair { 
                first: test,
                second: Pir(box TokenPair {
                    first: then,
                    second: _else })
                }) => {
                    let test = self.eval(test)?;
                    if let Lit(Bool(b)) = test {
                        if b { return self.eval(then); }
                        else { return self.eval(_else); }
                    } else {
                        return Err(
                            EvalError::typ(format!("Test branch of `if` should return bool, not {}", test)));
                    }
                },
            _ => return Err(
                EvalError::syntax("Need 'test, then, else' branches for `if`".to_string())
            )
        }
    }

    /// The evaluation method
    pub fn eval(&mut self, expr: TokenPairItem) -> Result<ObjectPairItem, EvalError> {
        use TokenPairItem::*;
        use ObjectPairItem::*;
        use Token::*;

        match expr {
            Tok(token) => {
                match token {
                    Word(word) => {
                        return self.find_object(&word).map_or(
                            Err(EvalError::name(format!("Name `{}` not found", word))), 
                            |x| Ok(x),
                        );
                    },
                    Table(t) => {
                        let mut ot_data = Vec::with_capacity(t.data.len());
                        for name in t.data.into_iter() {
                            ot_data.push(self.find_object(&name).map_or(Err(EvalError::name(format!("Name `{}` not found", name)), 
                                ), |x| Ok(x))?);
                        }
                        return Ok(Lit(Literal::Table(Box::new(
                            table::Table { data: ot_data, rows: t.rows, cols: t.cols }
                        ))));

                    },
                    other => {
                        return Ok(Lit(other.try_into().unwrap()));
                    }
                }
            },
            Pir(pair) => {
                if let Tok(Word(word)) = &pair.first {
                    match &word[..] {
                        "def" => {
                            self.define(pair.second)?;
                            return Ok(Lit(Literal::Nil));
                        },
                        "lambda" => {
                            let func = self.lambda(pair.second)?;
                            return Ok(Func(Rc::new(func)));
                        },
                        "if" => {
                            return self._if(pair.second);
                        },
                        _ => {},
                    }
                }
                let first = self.eval(pair.first.clone())?;

                match first {
                    Func(func) => 
                        return self.apply_func(&func, pair.second.clone()),
                        
                    BuiltinFunc(bfunc) => 
                        return self.apply_builtin_func(&bfunc, pair.second.clone()),
                
                    _ => return Ok(List(Box::new(ObjectPair { first, second: self.eval(pair.second)? }))),
                }
            }
        }
    }
}