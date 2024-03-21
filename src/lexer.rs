/// lexer.rs implements the lexer of interpreter.
use crate::lexeme::{char::Char, id::ID, keyword::Keyword, number::Number, op::Op, Type, Value};
use crate::token::Token;
use crate::{error::Error, utils};

#[derive(Debug)]
pub struct Lexer {
    code: Vec<char>, // The content of the input code.
    pos: usize,      // The current position of the input code.
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            code: text.chars().collect::<Vec<char>>(),
            pos: 0,
        }
    }

    pub fn current_char(&self) -> char {
        if self.pos >= self.code.len() {
            return '\0';
        }

        self.code[self.pos]
    }

    /// advance change lexer's current_char to the next char and add one to pos.
    pub fn advance(&mut self) {
        if self.pos >= self.code.len() {
            return;
        }

        self.pos += 1;
    }

    pub fn skip_whitespace_or_lf(&mut self) {
        while Char::Whitespace.equal_value(self.current_char())
            || Char::LF.equal_value(self.current_char())
        {
            self.advance();
        }
    }

    pub fn skip_comment(&mut self) {
        while self.current_char() != '}' {
            self.advance();
        }
        self.advance();
    }

    pub fn is_digit(&mut self) -> bool {
        match self.current_char().to_digit(10) {
            Option::Some(val) => val < 10,
            Option::None => false,
        }
    }

    pub fn number(&mut self) -> Result<Number, Error> {
        let mut val = String::from("");

        while self.is_digit() {
            val.push(self.current_char());
            self.advance();
        }

        if self.current_char() != '.' {
            return Ok(Number::Int(val));
        }

        val.push(self.current_char());
        self.advance();

        if !self.is_digit() {
            return Err(Error::InvalidSyntax);
        }

        while self.is_digit() {
            val.push(self.current_char());
            self.advance();
        }

        Ok(Number::Real(val))
    }

    /// peek returns the next char without increasing self.pos.
    pub fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos >= self.code.len() {
            None
        } else {
            Some(self.code[peek_pos])
        }
    }

    /// _id handles identifiers and reserved keywords.
    pub fn _id(&mut self) -> Token {
        let mut val = String::from("");

        while !Char::EOF.equal_value(self.current_char()) && utils::isalnum(&self.current_char()) {
            val.push(self.current_char());
            self.advance();
        }

        for k in vec![Keyword::Begin, Keyword::End, Keyword::Program] {
            if k.equal_value(&val) {
                return Token::new(k.r#type(), k.value());
            }
        }

        Token::new(ID, &val)
    }

    /// print_all_token prints all tokens of the input code.
    pub fn print_all_token(&mut self) {
        let mut token: Token;
        loop {
            token = match self.get_next_token() {
                Err(e) => {
                    println!("get_next_token failed, error: {}", e);
                    return;
                }
                Ok(t) => t,
            };
            println!("{:?}", token);

            if Char::EOF.equal_type(token.r#type()) {
                break;
            }
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        while !Char::EOF.equal_value(self.current_char()) {
            if Char::Whitespace.equal_value(self.current_char())
                || Char::LF.equal_value(self.current_char())
            {
                self.skip_whitespace_or_lf();
                continue;
            }

            if Char::LeftBracket.equal_type(self.current_char()) {
                self.advance();
                self.skip_comment();
                continue;
            }

            if utils::isalnum(&self.current_char()) {
                return Ok(self._id());
            }

            if self.is_digit() {
                match self.number() {
                    Err(e) => return Err(e),
                    Ok(n) => return Ok(Token::new(n.r#type(), n.value())),
                }
            }

            if Char::Colon.equal_value(self.current_char())
                && self.peek().expect(Error::InvalidSyntax.as_str()) == '='
            {
                self.advance();
                self.advance();
                return Ok(Token::new(Op::Assign.r#type(), Op::Assign.value()));
            }

            for op in vec![Op::Add, Op::Sub, Op::Mul, Op::Div] {
                if op.equal_value(self.current_char()) {
                    self.advance();
                    return Ok(Token::new(op.r#type(), op.value()));
                }
            }

            for c in vec![
                Char::Semi,
                Char::Dot,
                Char::LeftBracket,
                Char::RightBracket,
                Char::Colon,
                Char::Comma,
            ] {
                if c.equal_value(self.current_char()) {
                    self.advance();
                    return Ok(Token::new(c.r#type(), c.value()));
                }
            }

            return Err(Error::InvalidSyntax);
        }

        Ok(Token::new(Char::EOF.r#type(), Char::EOF.value()))
    }
}
