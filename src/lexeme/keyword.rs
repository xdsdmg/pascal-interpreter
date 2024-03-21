use crate::lexeme::{Type, Value};

pub enum Keyword {
    Begin,   // "BEGIN"
    End,     // "END"
    Program, // "PROGRAM"
    Var,     // "VAR"
}

impl Type for Keyword {
    fn r#type(&self) -> &'static str {
        match self {
            Keyword::Begin => "reserved keyword BEGIN",
            Keyword::End => "reserved keyword END",
            Keyword::Program => "reserved keyword PROGRAM",
            Keyword::Var => "reserved keyword VAR",
        }
    }
}

impl Value for Keyword {
    fn value(&self) -> &'static str {
        match self {
            Keyword::Begin => "BEGIN",
            Keyword::End => "END",
            Keyword::Program => "PROGRAM",
            Keyword::Var => "VAR",
        }
    }
}
