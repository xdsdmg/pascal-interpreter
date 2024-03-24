use crate::lexer::lexeme::{Type, Value};

pub enum Op {
    Add,    // "+"
    Sub,    // "-"
    Mul,    // "*"
    Div,    // "/"
    Assign, // ":="
}

impl Type for Op {
    fn r#type(&self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Mul => "mul",
            Op::Div => "div",
            Op::Assign => "assign",
        }
    }
}

impl Value for Op {
    fn value(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Assign => ":=",
        }
    }
}
