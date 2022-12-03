use crate::eval::Literal;

pub const EXPORTS: [(Literal, &'static str); 2] = [
    (Literal::Float(std::f64::consts::E), "e"),
    (Literal::Float(std::f64::consts::PI), "pi"),
];
