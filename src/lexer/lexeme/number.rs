use crate::lexer::lexeme::{Type, Value};

#[derive(PartialEq)]
pub enum NumberType {
    Integer,
    Real,
}

impl Type for NumberType {
    fn r#type(&self) -> &'static str {
        match self {
            NumberType::Integer => "INTEGER",
            NumberType::Real => "REAL",
        }
    }
}

pub enum Number {
    Integer(String),
    Real(String),
}

impl Type for Number {
    fn r#type(&self) -> &'static str {
        match self {
            Number::Integer(_) => NumberType::Integer.r#type(),
            Number::Real(_) => NumberType::Real.r#type(),
        }
    }
}

impl Value for Number {
    fn value(&self) -> &str {
        let s = match self {
            Number::Integer(s) => s,
            Number::Real(s) => s,
        };
        &s
    }
}
