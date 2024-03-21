use crate::lexeme::{Type, Value};

pub enum Char {
    Whitespace,   // " "
    LF,           // "\n"
    LeftBracket,  // "("
    RightBracket, // ")"
    Semi,         // ";"
    Dot,          // "."
    EOF,          // "\0"
    Colon,        // ":"
    Comma,        // ","
}

impl Value for Char {
    fn value(&self) -> &'static str {
        match self {
            Char::Whitespace => " ",
            Char::LF => "\n",
            Char::LeftBracket => "(",
            Char::RightBracket => ")",
            Char::Semi => ";",
            Char::Dot => ".",
            Char::EOF => "\0",
            Char::Colon => ":",
            Char::Comma => ",",
        }
    }
}

impl Type for Char {
    fn r#type(&self) -> &'static str {
        match self {
            Char::Whitespace => "whitespace",
            Char::LF => "lf",
            Char::LeftBracket => "left bracket",
            Char::RightBracket => "right bracket",
            Char::Semi => "semicolon",
            Char::Dot => "dot",
            Char::EOF => "eof",
            Char::Colon => "colon",
            Char::Comma => "comma",
        }
    }
}
