use crate::{error::Error, global_scope::global_scope_print, lexer::Lexer, parser::Parser};

pub struct Interpreter {
    pub parser: Parser,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Interpreter, Error> {
        let lexer = Lexer::new(code);
        let parser = match Parser::new(lexer) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        Ok(Interpreter { parser })
    }

    #[allow(dead_code)]
    pub fn print_all_token(&mut self) {
        self.parser.lexer.print_all_token()
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let root = match self.parser.parse() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let result = match root.visit() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        if let Some(r) = result {
            println!("[execute] result: {}", r);
        }

        global_scope_print();

        Ok(())
    }
}
