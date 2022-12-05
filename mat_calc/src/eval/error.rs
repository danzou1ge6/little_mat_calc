use mat::error::MatError;

#[derive(Debug)]
pub enum EvalError {
    SyntaxError(Vec<String>),
    NameError(Vec<String>),
    TypeError(Vec<String>),
    ValueError(Vec<String>),
    RecursionError(Vec<String>),
    ZeroDivisionError(Vec<String>),
}

impl EvalError {
    pub fn syntax(msg: String) -> Self {
        Self::SyntaxError(vec![msg])
    }
    pub fn name(msg: String) -> Self {
        Self::NameError(vec![msg])
    }
    pub fn typ(msg: String) -> Self {
        Self::TypeError(vec![msg])
    }
    pub fn value(msg: String) -> Self {
        Self::ValueError(vec![msg])
    }
    pub fn recursion(msg: String) -> Self {
        Self::RecursionError(vec![msg])
    }
    pub fn zero_division(msg: String) -> Self {
        Self::ZeroDivisionError(vec![msg])
    }

    pub fn cat_msg(self, msg: String) -> Self {
        match self {
            Self::SyntaxError(mut stack) => Self::SyntaxError({
                stack.push(msg);
                stack
            }),
            Self::NameError(mut stack) => Self::NameError({
                stack.push(msg);
                stack
            }),
            Self::TypeError(mut stack) => Self::TypeError({
                stack.push(msg);
                stack
            }),
            Self::ValueError(mut stack) => Self::ValueError({
                stack.push(msg);
                stack
            }),
            Self::RecursionError(mut stack) => Self::RecursionError({
                stack.push(msg);
                stack
            }),
            Self::ZeroDivisionError(mut stack) => Self::ZeroDivisionError({
                stack.push(msg);
                stack
            }),
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, stack) = match self {
            Self::SyntaxError(stack) => ("SyntaxError", stack),
            Self::NameError(stack) => ("NameError", stack),
            Self::TypeError(stack) => ("TypeError", stack),
            Self::ValueError(stack) => ("ValueError", stack),
            Self::RecursionError(stack) => ("RecursionError", stack),
            Self::ZeroDivisionError(stack) => ("ZeroDivisionError", stack),
        };

        write!(f, "{}\n    {}", name, stack.join("\n    "))
    }
}

impl std::error::Error for EvalError {}

impl From<MatError> for EvalError {
    fn from(value: MatError) -> Self {
        Self::value(format!("{value}"))
    }
}
