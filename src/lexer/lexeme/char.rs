use crate::lexer::lexeme::{Type, Value};

pub enum Char {
    Whitespace, // " "
    LF,         // "\n"
    LeftParen,  // "("
    RightParen, // ")"
    Semi,       // ";"
    Dot,        // "."
    EOF,        // "\0"
    Colon,      // ":"
    Comma,      // ","
    LeftBrace,  // "{"
    RightBrace, // "}"
}

impl Char {
    pub fn char(&self) -> char {
        match self {
            Char::Whitespace => ' ',
            Char::LF => '\n',
            Char::LeftParen => '(',
            Char::RightParen => ')',
            Char::Semi => ';',
            Char::Dot => '.',
            Char::EOF => '\0',
            Char::Colon => ':',
            Char::Comma => ',',
            Char::LeftBrace => '{',
            Char::RightBrace => '}',
        }
    }
}

impl Value for Char {
    fn value(&self) -> &'static str {
        match self {
            Char::Whitespace => " ",
            Char::LF => "\n",
            Char::LeftParen => "(",
            Char::RightParen => ")",
            Char::Semi => ";",
            Char::Dot => ".",
            Char::EOF => "\0",
            Char::Colon => ":",
            Char::Comma => ",",
            Char::LeftBrace => "{",
            Char::RightBrace => "}",
        }
    }
}

impl Type for Char {
    fn r#type(&self) -> &'static str {
        match self {
            Char::Whitespace => "whitespace",
            Char::LF => "lf",
            Char::LeftParen=> "left paren",
            Char::RightParen=> "right paren",
            Char::Semi => "semicolon",
            Char::Dot => "dot",
            Char::EOF => "eof",
            Char::Colon => "colon",
            Char::Comma => "comma",
            Char::LeftBrace => "left brace",
            Char::RightBrace => "right brace",
        }
    }
}
