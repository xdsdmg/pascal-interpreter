#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    #[test]
    fn test_lexer() {
        /*
         program Main;

         procedure Alpha(a : integer; b : integer);
         var x : integer;

            procedure Beta(a : integer; b : integer);
            var x : integer;
            begin
               x := a * 10 + b * 2;
            end;

         begin
            x := (a + b ) * 2;

            Beta(5, 10);      { procedure call }
         end;

         begin { Main }

            Alpha(3 + 5, 7);  { procedure call }

         end.  { Main }
        */

        /*
         BEGIN
            BEGIN
               number := 2;
               a := number;
               b := 10 * a + 10 * number / 4;
               c := a - - b
            END;
            x := 11;
         END.
        */

        let code = "
PROGRAM Part10AST;
VAR
   a, b : INTEGER;
   y    : REAL;

BEGIN {Part10AST}
   a := 2;
   b := 10 * a + 10 * a DIV 4;
   y := 20 / 7 + 3.14;
   z := -2;
END.  {Part10AST}
            ";

        let mut lexer = Lexer::new(code);

        lexer.print_all_token();
    }
}
