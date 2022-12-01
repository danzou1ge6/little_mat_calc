pub mod token {
    use crate::mat_wrap::MatrixWrap;
    use mat::Rational;
    use std::fmt;

    #[derive(Debug, Clone)]
    /// Abstraction of *Pieces* in Scheme
    pub enum Token {
        /// Rational
        Rat(Rational),
        /// Variables, keywords, etc.
        /// For example, `a`, `define`, `+`
        Word(String),
        /// Float, eg. `1.2`
        Float(f64),
        /// Matrix
        Matrix(MatrixWrap),
        Bool(bool),
        /// `nil`
        Nil,
    }

    impl fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Token::*;
            match self {
                Rat(r) => r.fmt(f),
                Word(w) => write!(f, "{}", w),
                Float(fl) => write!(f, "{}", fl),
                Matrix(m) => write!(f, "{m}"),
                Nil => write!(f, "nil"),
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

    impl From<&str> for Token {
        /// Construct a [`Token`] from [`&str`],
        /// which means *Tokenizing* a *Piece*
        ///
        /// This method won't panic.
        fn from(s: &str) -> Self {
            if let Ok(r) = s.try_into() {
                return Token::Rat(r);
            }
            if let Ok(float) = s.parse::<f64>() {
                return Token::Float(float);
            }
            if s == "nil" {
                return Token::Nil;
            }
            if s == "#t" {
                return Token::Bool(true);
            }
            if s == "#f" {
                return Token::Bool(false);
            }

            return Token::Word(s.to_string());
        }
    }
}

mod pair {
    use super::token::*;
    use std::fmt;

    #[derive(Debug)]
    /// An item that may appear in a [`Pair`].
    ///
    /// It can be a [`Pair`] itself, or represents a [`Token`]
    pub enum TokenPairItem {
        Tok(Token),
        Pir(Box<TokenPair>),
    }

    impl Default for TokenPairItem {
        fn default() -> Self {
            Self::Tok(Token::Nil)
        }
    }

    impl fmt::Display for TokenPairItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TokenPairItem::Tok(token) => {
                    write!(f, "{}", token)
                }
                TokenPairItem::Pir(pair) => {
                    write!(f, "({} {})", pair.first, pair.second)
                }
            }
        }
    }

    #[derive(Debug)]
    /// A scheme Pair
    pub struct TokenPair {
        pub first: TokenPairItem,
        pub second: TokenPairItem,
    }

    impl fmt::Display for TokenPair {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} {})", self.first, self.second)
        }
    }

    impl Clone for TokenPair {
        fn clone(&self) -> Self {
            Self {
                first: self.first.clone(),
                second: self.second.clone(),
            }
        }
    }

    impl Clone for TokenPairItem {
        fn clone(&self) -> Self {
            use TokenPairItem::*;
            match self {
                Tok(tok) => Tok(tok.clone()),
                Pir(pair) => {
                    let cloned_pair = (**pair).clone();
                    Pir(Box::new(cloned_pair))
                }
            }
        }
    }
}

mod parsing {

    use crate::mat_wrap::MatrixWrap;

    use super::pair::*;
    /// Parsing splitted *Pieces* into [`Pair`]

    #[derive(Debug)]
    /// Thrown when something goes wrong in the *Parsing* process
    pub struct ParseError {
        pub msg: String,
    }

    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ParseError: {}", self.msg)
        }
    }

    /// Represent the state of the [`TokenPairParser`]
    ///
    /// `Pending` means the expression is not complete, and parse needs to be called again
    pub enum PendingResult<T, E> {
        Ok(T),
        Pending,
        Err(E),
    }

    /// Takes the last two entries from `stack` and *pair* them
    fn stack_reduce(stack: &mut Vec<TokenPairItem>) -> Result<(), ParseError> {
        let second = match stack.pop() {
            Some(v) => v,
            None => {
                return Err(ParseError {
                    msg: String::from("Too many `)` 1"),
                })
            }
        };
        let first = match stack.pop() {
            Some(v) => v,
            None => {
                return Err(ParseError {
                    msg: String::from("Too many `)` 2"),
                })
            }
        };
        let new = TokenPair { first, second };
        stack.push(TokenPairItem::Pir(Box::new(new)));

        return Ok(());
    }

    /// Increase the number of the vector `stack`
    fn increase_last_cnt(cnt_stack: &mut Vec<usize>) -> PendingResult<(), ParseError> {
        use PendingResult::*;

        match cnt_stack.last_mut() {
            Some(val) => *val += 1,
            None => {
                return Err(ParseError {
                    msg: String::from("Expression must begin with `(`"),
                });
            }
        }
        return Ok(());
    }

    /// The parser to parse `Pieces` into [`TokenPairItem`]
    pub struct TokenPairParser {
        stack: Vec<TokenPairItem>,
        cnt_stack: Vec<usize>,
        mat_buf: Vec<String>,
        in_mat: bool,
    }

    impl TokenPairParser {
        pub fn new() -> Self {
            Self {
                stack: Vec::new(),
                cnt_stack: Vec::new(),
                mat_buf: Vec::new(),
                in_mat: false,
            }
        }
        /// If the input is not complete expression, it'll return a pending;
        /// If the input is already complete, it'll empty its internal state and get ready for
        /// next parsing
        pub fn parse(&mut self, v: Vec<&str>) -> PendingResult<TokenPairItem, ParseError> {
            use super::Token;
            use PendingResult::*;

            for split in v {
                match split {
                    "(" => {
                        self.cnt_stack.push(0);
                    }
                    ")" => {
                        let mut reduction_needed = match self.cnt_stack.pop() {
                            Some(v) => v,
                            None => {
                                return Err(ParseError {
                                    msg: String::from("Too many `)` 4"),
                                })
                            }
                        };
                        if reduction_needed == 0 {
                            return Err(ParseError {
                                msg: String::from("Empty `()` not allowed"),
                            });
                        } else {
                            reduction_needed -= 1
                        };

                        for _ in 0..reduction_needed {
                            if let Result::Err(e) = stack_reduce(&mut self.stack) {
                                return Err(e);
                            }
                        }
                        if self.cnt_stack.len() == 0 {
                            break;
                        }
                        if let Err(e) = increase_last_cnt(&mut self.cnt_stack) {
                            return Err(e);
                        }
                    }
                    "[" => {
                        self.mat_buf.clear();
                        self.in_mat = true;
                    }
                    "]" => {
                        self.in_mat = false;
                        let matrix_wrap: MatrixWrap =
                            match (&mut self.mat_buf.iter().map(|x| &x[..])
                                as &mut dyn Iterator<Item = &str>)
                                .try_into()
                                .map_err(|e| ParseError {
                                    msg: format!("{:?}", e),
                                }) {
                                Result::Ok(mr) => mr,
                                Result::Err(e) => return PendingResult::Err(e),
                            };
                        self.stack
                            .push(TokenPairItem::Tok(Token::Matrix(matrix_wrap)));
                        if let Err(e) = increase_last_cnt(&mut self.cnt_stack) {
                            return Err(e);
                        }
                    }
                    other => {
                        if self.in_mat {
                            self.mat_buf.push(other.to_string());
                        } else {
                            self.stack.push(TokenPairItem::Tok(other.into()));
                            if let Err(e) = increase_last_cnt(&mut self.cnt_stack) {
                                return Err(e);
                            }
                        }
                    }
                }
            }

            if self.stack.len() == 1 {
                let ret = self.stack.pop().unwrap();

                // empty the current stack
                self.stack = Vec::new();
                self.cnt_stack = Vec::new();

                return Ok(ret);
            } else {
                return Pending;
            }
        }
    }

    #[cfg(test)]
    mod test {

        use super::super::Token;
        use super::*;
        use crate::mat_wrap::MatrixWrap;
        use mat::Rational;

        impl<T, E: std::fmt::Debug> PendingResult<T, E> {
            fn unwrap(self) -> T {
                use PendingResult::*;
                match self {
                    Ok(v) => v,
                    Pending => panic!("Called `unwrap` on a pending `PendingResult`"),
                    Err(e) => panic!("Called `unwrap` on a error `PendignResult` {:?}", e),
                }
            }
        }

        #[test]
        fn test() {
            let pieces = vec![
                "(", "(", "x", "y", ")", "(", "a", "b", "c", ")", "p", "q", ")",
            ];
            let mut parser = TokenPairParser::new();
            let pair_item = parser.parse(pieces).unwrap();

            let formatted = format!("{}", pair_item);
            assert_eq!(&formatted, "((x y) ((a (b c)) (p q)))");
        }

        #[test]
        fn test_pending() {
            let pieces = vec!["(", "(", "x", "y", ")", "(", "a", "b", "c", ")", "p", "q"];
            let mut parser = TokenPairParser::new();
            match parser.parse(pieces) {
                PendingResult::Pending => {}
                _ => panic!("should be pending"),
            };

            let pair_item = parser.parse(vec![")"]).unwrap();
            let formatted = format!("{}", pair_item);
            assert_eq!(&formatted, "((x y) ((a (b c)) (p q)))");
        }

        #[test]
        fn test_matrix() {
            let pieces = vec!["(", "x", "[", "1", ";", "]", ")"];
            let mut parser = TokenPairParser::new();
            let pair_item = parser.parse(pieces).unwrap();

            match pair_item {
                TokenPairItem::Pir(box TokenPair {
                    first: TokenPairItem::Tok(Token::Word(word)),
                    second: TokenPairItem::Tok(Token::Matrix(MatrixWrap::Rat(m))),
                }) => {
                    assert_eq!(word, "x");
                    assert_eq!(m.dimensions(), (1, 1));
                    assert_eq!(m.get(0, 0).unwrap(), &Rational(1, 1));
                }
                _ => panic!("Unexpected pair: {pair_item}"),
            }
        }
    }
}

pub use pair::TokenPair;
pub use pair::TokenPairItem;
pub use parsing::ParseError;
pub use parsing::PendingResult;
pub use parsing::TokenPairParser;
pub use token::Token;
