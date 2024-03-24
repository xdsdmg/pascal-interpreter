use crate::{error::Error, global_scope::global_scope_print, lexer::Lexer, parser::Parser};

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

        match root.visit() {
            Ok(info) => {
                if let Some(v) = info.value() {
                    println!("[interpreter] [execute] result: {}", v);
                }
            }
            Err(e) => return Err(e),
        };

        global_scope_print();

        Ok(())
    }
}
