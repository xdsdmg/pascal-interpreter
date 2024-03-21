/// parser.rs implements the parsing functionality Pascal's BNF.
use crate::ast::assign::Assign;
use crate::ast::compound::Compound;
use crate::ast::program::Program;
use crate::ast::{bin_op::BinOp, no_op::NoOp, num::Num, unary_op::UnaryOp, var::Var, Node};
use crate::error::Error;
use crate::lexeme::{
    char::Char, id::ID, keyword::Keyword, number::NumberType, op::Op, Type, Value,
};
use crate::lexer::Lexer;
use crate::token::Token;
use std::rc::Rc;

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Parser, Error> {
        let current_token: Token;

        match lexer.get_next_token() {
            Ok(token) => current_token = token,
            Err(e) => return Err(e),
        }

        Ok(Parser {
            lexer,
            current_token,
        })
    }

    // parse parses code into AST.
    pub fn parse(&mut self) -> Result<Rc<dyn Node>, Error> {
        let node = match self.program() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        if !Char::EOF.equal_type(self.current_token.r#type()) {
            return Err(Error::InvalidSyntax);
        }

        Ok(node)
    }

    /// eat changes parser's current_token to the next token.
    pub fn eat(&mut self, token_type: &str) {
        println!(
            "current token: {:?}, token_type: {}",
            self.current_token, token_type
        );

        if self.current_token.r#type() != token_type {
            panic!(
                "current token: {:?}, token_type: {}, error: {}",
                self.current_token,
                token_type,
                Error::InvalidSyntax.to_string()
            );
        }

        match self.lexer.get_next_token() {
            Ok(token) => self.current_token = token,
            Err(e) => panic!("{}", e),
        }
    }

    /// program implements the below BNF:
    /// program: PROGRAM variable SEMI block DOT
    pub fn program(&mut self) -> Result<Rc<dyn Node>, Error> {
        self.eat(Keyword::Program.r#type());

        let name = self.variable().optional();

        self.eat(Char::Semi.r#type());

        /* block */

        self.eat(Char::Dot.r#type());

        Ok(Rc::new(Program::new(&name)))

        // match self.compound_statement() {
        //     Ok(n) => {
        //         self.eat(Char::Dot.r#type());
        //         Ok(n)
        //     }
        //     Err(e) => Err(e),
        // }
    }

    /// block implements the below BNF:
    /// block: declarations compound_statement
    pub fn block(&mut self) -> Result<Rc<dyn Node>, Error> {}

    /// declarations implements the below BNF:
    /// declarations: VAR (variable_declaration SEMI)+
    ///             | empty
    pub fn declarations(&mut self) {}

    /// variable_declaration implements the below BNF:
    /// variable_declaration: ID (COMMA ID)* COLON type_spec
    pub fn variable_declaration(&mut self) {
        let v = self.variable();
    }

    /// compound_statement implements the below BNF:
    /// compound_statement: BEGIN statement_list END
    pub fn compound_statement(&mut self) -> Result<Rc<dyn Node>, Error> {
        let mut children = Vec::<Rc<dyn Node>>::new();

        self.eat(Keyword::Begin.r#type());

        match self.statement_list() {
            Ok(nodes) => nodes.iter().for_each(|n| children.push(n.clone())),
            Err(e) => return Err(e),
        };

        self.eat(Keyword::End.r#type());

        Ok(Rc::new(Compound::new(children)))
    }

    /// statement_list implements the below BNF:
    /// statement_list: statement | statement SEMI statement_list
    pub fn statement_list(&mut self) -> Result<Vec<Rc<dyn Node>>, Error> {
        let mut result = Vec::<Rc<dyn Node>>::new();

        match self.statement() {
            Ok(n) => result.push(n),
            Err(e) => return Err(e),
        };

        while Char::Semi.equal_type(self.current_token.r#type()) {
            self.eat(Char::Semi.r#type());
            match self.statement() {
                Ok(n) => result.push(n),
                Err(e) => return Err(e),
            };
        }

        Ok(result)
    }

    /// statement implements the below BNF:
    /// statement: compound_statement | assignment_statement | empty
    pub fn statement(&mut self) -> Result<Rc<dyn Node>, Error> {
        if Keyword::Begin.equal_type(self.current_token.r#type()) {
            self.compound_statement()
        } else if self.current_token.r#type() == ID {
            self.assginment_statement()
        } else {
            Ok(self.empty())
        }
    }

    /// assignment_statement implements the below BNF:
    /// assignment_statement: variable ASSIGN expr
    pub fn assginment_statement(&mut self) -> Result<Rc<dyn Node>, Error> {
        let left = self.variable();

        let token = self.current_token.clone();

        self.eat(Op::Assign.r#type());

        let right = match self.expr() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let result = Assign::new(&left.optional(), &token, right);

        Ok(Rc::new(result))
    }

    /// variable implements the below BNF:
    /// variable: ID
    pub fn variable(&mut self) -> Rc<dyn Node> {
        let result = Var::new(&self.current_token);
        self.eat(ID);
        Rc::new(result)
    }

    /// An empty production
    pub fn empty(&mut self) -> Rc<dyn Node> {
        Rc::new(NoOp::new())
    }

    /// factor implements the below BNF:
    /// factor: PLUS factor | MINUS factor | INTEGER | LPAREN expr RPAREN | variable
    pub fn factor(&mut self) -> Result<Rc<dyn Node>, Error> {
        if NumberType::Int.equal_type(self.current_token.r#type()) {
            let val = match self.current_token.value().parse::<i32>() {
                Ok(v) => v,
                Err(_) => return Err(Error::InvalidSyntax),
            };
            self.eat(NumberType::Int.r#type());

            let token = Token::new(NumberType::Int.r#type(), &val.to_string());
            let node = Rc::new(Num::new(token, val));

            Ok(node)
        } else if Char::LeftBracket.equal_type(&self.current_token.r#type()) {
            self.eat(Char::LeftBracket.r#type());

            let node: Rc<dyn Node> = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };

            self.eat(Char::RightBracket.r#type());

            Ok(node)
        } else if Op::Add.equal_type(&self.current_token.r#type()) {
            self.eat(Op::Add.r#type());

            let node: Rc<dyn Node> = match self.factor() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };

            Ok(Rc::new(UnaryOp::new(
                Token::new(Op::Add.r#type(), Op::Add.value()),
                node,
            )))
        } else if Op::Sub.equal_type(&self.current_token.r#type()) {
            self.eat(Op::Sub.r#type());

            let node: Rc<dyn Node> = match self.factor() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };

            Ok(Rc::new(UnaryOp::new(
                Token::new(Op::Sub.r#type(), Op::Sub.value()),
                node,
            )))
        } else if self.current_token.r#type() == ID {
            Ok(self.variable())
        } else {
            Err(Error::InvalidSyntax)
        }
    }

    /// term implements the below BNF:
    /// term: factor ((MUL | DIV) factor)*
    pub fn term(&mut self) -> Result<Rc<dyn Node>, Error> {
        let node = match self.factor() {
            Ok(node_) => node_,
            Err(e) => return Err(e),
        };

        let mut result = node;

        while Op::Mul.equal_type(&self.current_token.r#type())
            || Op::Div.equal_type(&self.current_token.r#type())
        {
            if Op::Mul.equal_type(&self.current_token.r#type()) {
                self.eat(Op::Mul.r#type());
                match self.factor() {
                    Ok(n) => {
                        let bin_op = BinOp::new(
                            result.clone(),
                            Token::new(Op::Mul.r#type(), Op::Mul.value()),
                            n,
                        );
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            } else if Op::Div.equal_type(&self.current_token.r#type()) {
                self.eat(Op::Div.r#type());
                match self.factor() {
                    Ok(n) => {
                        let bin_op = BinOp::new(
                            result.clone(),
                            Token::new(Op::Div.r#type(), Op::Div.value()),
                            n,
                        );
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(result)
    }

    /// expr implements the below BNF:
    /// expr: term ((PLUS | MINUS) term)*
    pub fn expr(&mut self) -> Result<Rc<dyn Node>, Error> {
        let node = match self.term() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let mut result = node;

        while Op::Add.equal_type(self.current_token.r#type())
            || Op::Sub.equal_type(self.current_token.r#type())
        {
            if Op::Add.equal_type(self.current_token.r#type()) {
                self.eat(Op::Add.r#type());
                match self.term() {
                    Ok(n) => {
                        let bin_op = BinOp::new(
                            result.clone(),
                            Token::new(Op::Add.r#type(), Op::Add.value()),
                            n,
                        );
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            } else if Op::Sub.equal_type(&self.current_token.r#type()) {
                self.eat(Op::Sub.r#type());
                match self.term() {
                    Ok(n) => {
                        let bin_op = BinOp::new(
                            result.clone(),
                            Token::new(Op::Sub.r#type(), Op::Sub.value()),
                            n,
                        );
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(result)
    }
}
