#[cfg(test)]
mod tests {
    use crate::global_scope::global_scope_print;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    #[test]
    fn test_parser() {
        let code = "
PROGRAM Part10AST;
VAR
   a, b : INTEGER;
   y    : REAL;

BEGIN {Part10AST}
   a := 2;
   b := 10 * a + 10 * a / 4;
   y := 20 / 7 + 3.14;
   z := -2;
END.  {Part10AST}
            ";

        let mut parser = Parser::new(Lexer::new(code));
        let root = match parser.parse() {
            Ok(n) => n,
            Err(e) => panic!("parse failed, error: {}", e),
        };

        let _ = root.visit();

        global_scope_print();
    }
}
