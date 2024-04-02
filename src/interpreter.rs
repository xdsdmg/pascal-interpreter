use crate::{error::Error, global_scope::Scope, lexer::Lexer, parser::Parser};
use std::{cell::RefCell, rc::Rc};

pub struct Interpreter {
    pub parser: Parser,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Interpreter, Error> {
        let lexer = Lexer::new(code);
        let parser = Parser::new(lexer);
        Ok(Interpreter { parser })
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let root = match self.parser.parse() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let scope = Rc::new(RefCell::new(Scope::new("base", None, 0)));
        match root.visit(scope) {
            Ok(info) => {
                if let Some(v) = info.value() {
                    println!("[interpreter] [execute] result: {}", v);
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
