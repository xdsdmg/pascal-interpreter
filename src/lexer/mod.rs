use self::lexeme::number::NumberType;
use crate::token::Token;
use crate::{error::Error, utils};
use lexeme::{char::Char, id::ID, keyword::Keyword, number::Number, op::Op, Type, Value};

pub mod lexeme;
mod test;

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

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        'l: while !Char::EOF.equal_value(self.current_char()) {
            /* Skip whitespace and '\n' */
            for c in [Char::Whitespace, Char::LF] {
                if c.equal_value(self.current_char()) {
                    self.advance();
                    continue 'l;
                }
            }

            /* Skip comments */
            if Char::LeftBrace.equal_value(self.current_char()) {
                self.advance();
                self.skip_comment();
                continue;
            }

            /* Identifier scan */
            if utils::isalnum(&self.current_char()) {
                return Ok(self._id());
            }

            /* Number scan */
            if utils::is_digit(&self.current_char()) {
                match self.number() {
                    Err(e) => return Err(self.wrap_error(e)),
                    Ok(n) => return Ok(Token::new(n.r#type(), n.value())),
                }
            }

            /* Assign (":=") scan */
            if Char::Colon.equal_value(self.current_char())
                && self.peek().expect(Error::InvalidSyntax.as_str()) == '='
            {
                self.advance();
                self.advance();
                return Ok(Token::new(Op::Assign.r#type(), Op::Assign.value()));
            }

            /* '+', '-', '*', '/' scan */
            for op in [Op::Add, Op::Sub, Op::Mul, Op::Div] {
                if op.equal_value(self.current_char()) {
                    self.advance();
                    return Ok(Token::new(op.r#type(), op.value()));
                }
            }

            /* ';', '.', '(', ')', ':', ',' scan */
            for c in [
                Char::Semi,
                Char::Dot,
                Char::LeftParen,
                Char::RightParen,
                Char::Colon, // Should after Assign.
                Char::Comma,
            ] {
                if c.equal_value(self.current_char()) {
                    self.advance();
                    return Ok(Token::new(c.r#type(), c.value()));
                }
            }

            return Err(self.wrap_error(Error::InvalidSyntax));
        }

        Ok(Token::new(Char::EOF.r#type(), Char::EOF.value()))
    }

    #[allow(dead_code)]
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
            println!("{}", token);

            if Char::EOF.equal_type(token.r#type()) {
                break;
            }
        }
    }

    fn current_char(&self) -> char {
        if self.pos >= self.code.len() {
            return Char::EOF.char();
        }

        self.code[self.pos]
    }

    /// advance change lexer's current_char to the next char and add one to pos.
    fn advance(&mut self) {
        if self.pos >= self.code.len() {
            return;
        }

        self.pos += 1;
    }

    fn skip_comment(&mut self) {
        while self.current_char() != Char::RightBrace.char() {
            self.advance();
        }
        self.advance();
    }

    fn number(&mut self) -> Result<Number, Error> {
        let mut val = String::from("");

        while utils::is_digit(&self.current_char()) {
            val.push(self.current_char());
            self.advance();
        }

        if !Char::Dot.equal_value(self.current_char()) {
            return Ok(Number::Integer(val));
        }

        val.push(self.current_char());
        self.advance();

        if !utils::is_digit(&self.current_char()) {
            return Err(Error::InvalidSyntax);
        }

        while utils::is_digit(&self.current_char()) {
            val.push(self.current_char());
            self.advance();
        }

        Ok(Number::Real(val))
    }

    /// peek returns the next char without increasing self.pos.
    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos >= self.code.len() {
            None
        } else {
            Some(self.code[peek_pos])
        }
    }

    /// _id handles identifiers and reserved keywords.
    fn _id(&mut self) -> Token {
        let mut val = String::from("");

        val.push(self.current_char());
        self.advance();

        while !Char::EOF.equal_value(self.current_char())
            && (utils::isalnum(&self.current_char()) || utils::is_digit(&self.current_char()))
        {
            val.push(self.current_char());
            self.advance();
        }

        for k in [Keyword::Begin, Keyword::End, Keyword::Program, Keyword::Var] {
            if k.equal_value(&val.to_ascii_uppercase()) {
                return Token::new(k.r#type(), k.value());
            }
        }

        for t in [NumberType::Integer, NumberType::Real] {
            if t.equal_type(&val) {
                return Token::new(t.r#type(), t.r#type());
            }
        }

        Token::new(ID, &val)
    }

    fn wrap_error(&self, err: Error) -> Error {
        println!(
            "[lexer] get the next token failed, current char: {}, ascii: {}, pos: {}",
            self.current_char(),
            self.current_char() as u32,
            self.pos
        );
        err
    }
}
