#[cfg(test)]
mod tests {
    use crate::global_scope::Scope;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use std::env;
    use std::fs;
    use std::{cell::RefCell, rc::Rc};
    #[test]
    fn test_parser() {
        let args: Vec<String> = env::args().collect();
        if args.len() == 0 {
            panic!("code file not found");
        }
        let filename = &args[args.len() - 1];

        let code =
            fs::read_to_string(filename).expect("Something went wrong when reading the file");

        let mut parser = Parser::new(Lexer::new(&code));
        let root = match parser.parse() {
            Ok(n) => n,
            Err(e) => panic!("parse failed, error: {}", e),
        };

        let scope = Rc::new(RefCell::new(Scope::new("base", None, 0)));
        let _ = root.visit(scope);
    }
}
