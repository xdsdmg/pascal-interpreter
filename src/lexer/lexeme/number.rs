use crate::{
    error::Error,
    lexer::lexeme::{Type, Value},
};

#[derive(PartialEq, Copy)]
pub enum NumberType {
    Integer,
    Real,
}

impl NumberType {
    pub fn to_number_type(s: &str) -> Result<NumberType, Error> {
        if s == NumberType::Integer.r#type() {
            return Ok(NumberType::Integer);
        } else if s == NumberType::Real.r#type() {
            return Ok(NumberType::Real);
        }
        return Err(Error::InvalidSyntax);
    }
}

impl Clone for NumberType {
    fn clone(&self) -> Self {
        match self {
            NumberType::Integer => NumberType::Integer,
            NumberType::Real => NumberType::Real,
        }
    }
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
