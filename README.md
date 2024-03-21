# pascal-interpreter

Pascal Interpreter

```
program: compound_statement DOT
program: PROGRAM variable SEMI block DOT
block: declarations compound_statement

declarations: VAR (variable_declaration SEMI)+
            | empty

variable_declaration: ID (COMMA ID)* COLON type_spec

compound_statement: BEGIN statement_list END

statement_list: statement
              | statement SEMI statement_list

statement: compound_statement
         | assignment_statement
         | empty

assignment_statement: variable ASSIGN expr

empty:

expr: term ((PLUS | MINUS) term)*

term: factor ((MUL | DIV) factor)*

factor: PLUS factor
      | MINUS factor
      | INTEGER
      | LPAREN expr RPAREN
      | variable

variable: ID
```

```
PROGRAM Part10AST;
VAR
   a, b : INTEGER;
   y    : REAL;

BEGIN {Part10AST}
   a := 2;
   b := 10 * a + 10 * a DIV 4;
   y := 20 / 7 + 3.14;
END.  {Part10AST}
```

## TODO

Optimize error message.
