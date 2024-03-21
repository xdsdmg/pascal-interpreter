use crate::lexeme::{Type, Value};

pub enum NumberType {
    Int,
    Real,
}

impl Type for NumberType {
    fn r#type(&self) -> &'static str {
        match self {
            NumberType::Int => "int",
            NumberType::Real => "real",
        }
    }
}

pub enum Number {
    Int(String),
    Real(String),
}

impl Type for Number {
    fn r#type(&self) -> &'static str {
        match self {
            Number::Int(_) => NumberType::Int.r#type(),
            Number::Real(_) => NumberType::Real.r#type(),
        }
    }
}

impl Value for Number {
    fn value(&self) -> &str {
        let s = match self {
            Number::Int(s) => s,
            Number::Real(s) => s,
        };

        &s
    }
}
