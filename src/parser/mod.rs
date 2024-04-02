use crate::ast::block::Block;
use crate::ast::declaration::Declaration;
use crate::ast::procedure::Procedure;
use crate::ast::var_decl::VarDecl;
use crate::ast::{
    assign::Assign, bin_op::BinOp, compound::Compound, integer::Integer, no_op::NoOp,
    program::Program, real::Real, unary_op::UnaryOp, var::Var, Node,
};
use crate::error::Error;
use crate::lexer::lexeme::{
    char::Char, id::ID, keyword::Keyword, number::NumberType, op::Op, Type, Value,
};
use crate::lexer::Lexer;
use crate::token::Token;
use std::rc::Rc;
use std::time::SystemTime;

mod tests;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            current_token: Token::new(Char::EOF.r#type(), Char::EOF.value()),
        }
    }

    // parse parses code into AST.
    pub fn parse(&mut self) -> Result<Rc<dyn Node>, Error> {
        let begin = SystemTime::now();

        /* Init current token */
        let token = match self.lexer.get_next_token() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        self.current_token = token;

        let node = match self.program() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        if !Char::EOF.equal_type(self.current_token.r#type()) {
            println!(
                "[parser] [parse] current token {} is not EOF",
                self.current_token
            );
            return Err(Error::InvalidSyntax);
        }

        println!(
            "[parser] [parse] time used: {}us",
            SystemTime::now().duration_since(begin).unwrap().as_micros()
        );

        Ok(node)
    }

    /// eat changes parser's current_token to the next token.
    fn eat(&mut self, token_type: &str) {
        println!("[eat] current token: {}", self.current_token);

        if self.current_token.r#type() != token_type {
            panic!(
                "[parser] [eat] current token {} did not match the required token type {}",
                self.current_token, token_type,
            );
        }

        match self.lexer.get_next_token() {
            Ok(token) => self.current_token = token,
            Err(e) => panic!(
                "[parser] [eat] get the next token failed, current token: {}, error: {}",
                self.current_token, e,
            ),
        }
    }

    /// BNF:
    /// program: (PROGRAM id SEMI)? block DOT
    fn program(&mut self) -> Result<Rc<dyn Node>, Error> {
        let mut name = String::from("");

        if Keyword::Program.equal_type(self.current_token.r#type()) {
            /* PROGRAM */
            self.eat(Keyword::Program.r#type());

            /* id */
            name = match self.variable().name() {
                Ok(name_op) => match name_op {
                    Some(name) => name,
                    None => {
                        println!(
                            "[parser] [program] variable's name not found, current token: {}",
                            self.current_token
                        );
                        return Err(Error::VarNotFound);
                    }
                },
                Err(e) => return Err(e),
            };

            /* SEMI */
            self.eat(Char::Semi.r#type());
        }

        /* block */
        let block = match self.block() {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        /* DOT */
        self.eat(Char::Dot.r#type());

        Ok(Rc::new(Program::new(&name, block)))
    }

    /// BNF:
    /// procedure: PROCEDURE id (LPAREN formal_parameter_list RPAREN)? SEMI block SEMI
    fn procedure(&mut self) -> Result<Procedure, Error> {
        self.eat(Keyword::Procedure.r#type());

        let name = match self.variable().name() {
            Ok(name_op) => match name_op {
                Some(name) => name,
                None => {
                    println!(
                        "[parser] [procedure] variable's name not found, current token: {}",
                        self.current_token
                    );
                    return Err(Error::VarNotFound);
                }
            },
            Err(e) => return Err(e),
        };

        let mut var_decl_list: Vec<Rc<VarDecl>> = Vec::new();
        if Char::LeftParen.equal_type(self.current_token.r#type()) {
            self.eat(Char::LeftParen.r#type());
            match self.formal_parameter_list() {
                Ok(vds) => vds.iter().for_each(|vd| var_decl_list.push(vd.clone())),
                Err(e) => return Err(e),
            };
            self.eat(Char::RightParen.r#type());
        }

        self.eat(Char::Semi.r#type());
        let block = match self.block() {
            Ok(block) => block,
            Err(e) => return Err(e),
        };
        self.eat(Char::Semi.r#type());

        Ok(Procedure::new(&name, var_decl_list, block))
    }

    /// BNF:
    /// formal_parameter_list: variable_declaration
    ///                      | variable_declaration SEMI formal_parameter_list
    fn formal_parameter_list(&mut self) -> Result<Vec<Rc<VarDecl>>, Error> {
        let mut var_decls: Vec<Rc<VarDecl>> = Vec::new();

        match self.variable_declaration() {
            Ok(var_decl) => var_decls.push(Rc::new(var_decl)),
            Err(e) => return Err(e),
        };

        if !Char::Semi.equal_type(self.current_token.r#type()) {
            return Ok(var_decls);
        }

        self.eat(Char::Semi.r#type());
        match self.formal_parameter_list() {
            Ok(vds) => vds.iter().for_each(|vd| var_decls.push(vd.clone())),
            Err(e) => return Err(e),
        }

        return Ok(var_decls);
    }

    /// BNF:
    /// block: declarations compound_statement
    fn block(&mut self) -> Result<Block, Error> {
        let declaration = match self.declarations() {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let cs = match self.compound_statement() {
            Ok(cs) => cs,
            Err(e) => return Err(e),
        };

        Ok(Block::new(declaration, cs))
    }

    /// BNF:
    /// declarations: (VAR (variable_declaration SEMI)+)* (procedure)*
    ///             | empty
    fn declarations(&mut self) -> Result<Declaration, Error> {
        let mut declaration = Declaration::new(Vec::new(), Vec::new());

        if !Keyword::Var.equal_type(self.current_token.r#type()) {
            return Ok(declaration);
        }

        while Keyword::Var.equal_type(self.current_token.r#type()) {
            self.eat(Keyword::Var.r#type());

            while self.current_token.r#type() == ID {
                match self.variable_declaration() {
                    Ok(vd) => declaration.var_decl_list_push(Rc::new(vd)),
                    Err(e) => return Err(e),
                };
                self.eat(Char::Semi.r#type());
            }
        }

        while Keyword::Procedure.equal_type(self.current_token.r#type()) {
            match self.procedure() {
                Ok(p) => declaration.procedure_list_push(Rc::new(p)),
                Err(e) => return Err(e),
            };
        }

        Ok(declaration)
    }

    /// BNF:
    /// variable_declaration: ID (COMMA ID)* COLON type_spec
    fn variable_declaration(&mut self) -> Result<VarDecl, Error> {
        let mut ids: Vec<String> = Vec::new();

        match self.variable().name() {
            Ok(name_op) => match name_op {
                Some(id) => ids.push(id),
                None => {
                    println!(
                        "[parser] [variable_declaration] variable's name not found, current token: {}",
                        self.current_token
                    );
                    return Err(Error::VarNotFound);
                }
            },
            Err(e) => return Err(e),
        };

        while Char::Comma.equal_type(self.current_token.r#type()) {
            self.eat(Char::Comma.r#type());
            match self.variable().name() {
                Ok(name_op) => match name_op {
                    Some(id) => ids.push(id),
                    None => {
                        println!(
                            "[parser] [variable_declaration] variable's name not found, current token: {}",
                            self.current_token
                        );
                        return Err(Error::VarNotFound);
                    }
                },
                Err(e) => return Err(e),
            };
        }

        self.eat(Char::Colon.r#type());

        let type_spec: NumberType;
        if NumberType::Integer.equal_type(self.current_token.r#type()) {
            self.eat(NumberType::Integer.r#type());
            type_spec = NumberType::Integer;
        } else if NumberType::Real.equal_type(self.current_token.r#type()) {
            self.eat(NumberType::Real.r#type());
            type_spec = NumberType::Real;
        } else {
            println!(
                "[parser] [variable_declaration] current token {} is invalid, Real or Integer is required",
                self.current_token
            );
            return Err(Error::InvalidSyntax);
        }

        Ok(VarDecl::new(ids, type_spec))
    }

    /// BNF:
    /// compound_statement: BEGIN statement_list END
    fn compound_statement(&mut self) -> Result<Compound, Error> {
        let mut children = Vec::<Rc<dyn Node>>::new();

        self.eat(Keyword::Begin.r#type());

        match self.statement_list() {
            Ok(nodes) => nodes.iter().for_each(|n| children.push(n.clone())),
            Err(e) => return Err(e),
        };

        self.eat(Keyword::End.r#type());

        Ok(Compound::new(children))
    }

    /// BNF:
    /// statement_list: statement | statement SEMI statement_list
    fn statement_list(&mut self) -> Result<Vec<Rc<dyn Node>>, Error> {
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

    /// BNF:
    /// statement: compound_statement | assignment_statement | procedure_call_statement | empty
    fn statement(&mut self) -> Result<Rc<dyn Node>, Error> {
        if Keyword::Begin.equal_type(self.current_token.r#type()) {
            match self.compound_statement() {
                Ok(cs) => Ok(Rc::new(cs)),
                Err(e) => return Err(e),
            }
        } else if self.current_token.r#type() == ID {
            self.assginment_statement()
        } else {
            Ok(self.empty())
        }
    }

    /// BNF:
    /// procedure_call_statement: id LPAREN (expr (COMMA expr)*)? RPAREN
    fn procedure_call(&mut self) -> Result<Rc<dyn Node>, Error> {
        todo!();
    }

    /// BNF:
    /// assignment_statement: variable ASSIGN expr
    fn assginment_statement(&mut self) -> Result<Rc<dyn Node>, Error> {
        let left = self.variable();

        self.eat(Op::Assign.r#type());

        let right = match self.expr() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let name = match left.name() {
            Ok(name_op) => match name_op {
                Some(name) => name,
                None => {
                    println!(
                        "[parser] [assginment_statement] variable's name not found, current token: {}",
                        self.current_token
                    );
                    return Err(Error::VarNotFound);
                }
            },
            Err(e) => return Err(e),
        };

        let result = Assign::new(&name, right);

        Ok(Rc::new(result))
    }

    /// BNF:
    /// variable: ID
    fn variable(&mut self) -> Rc<dyn Node> {
        let result = Var::new(self.current_token.value());
        self.eat(ID);
        Rc::new(result)
    }

    /// An empty production
    fn empty(&mut self) -> Rc<dyn Node> {
        Rc::new(NoOp::new())
    }

    /// BNF:
    /// factor: PLUS factor | MINUS factor | INTEGER | LPAREN expr RPAREN | variable
    fn factor(&mut self) -> Result<Rc<dyn Node>, Error> {
        if NumberType::Integer.equal_type(self.current_token.r#type()) {
            let val = match self.current_token.value().parse::<i32>() {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "[parser] [factor] parse num {} failed, current token: {}, error: {}",
                        self.current_token.value(),
                        self.current_token,
                        e,
                    );
                    return Err(Error::InvalidSyntax);
                }
            };
            self.eat(NumberType::Integer.r#type());
            Ok(Rc::new(Integer::new(val)))
        } else if NumberType::Real.equal_type(self.current_token.r#type()) {
            let val = match self.current_token.value().parse::<f32>() {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "[parser] [factor] parse num {} failed, current token: {}, error: {}",
                        self.current_token.value(),
                        self.current_token,
                        e,
                    );
                    return Err(Error::InvalidSyntax);
                }
            };
            self.eat(NumberType::Real.r#type());
            Ok(Rc::new(Real::new(val)))
        } else if Char::LeftParen.equal_type(self.current_token.r#type()) {
            self.eat(Char::LeftParen.r#type());
            let node: Rc<dyn Node> = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e),
            };
            self.eat(Char::RightParen.r#type());
            Ok(node)
        } else if Op::Add.equal_type(self.current_token.r#type())
            || Op::Sub.equal_type(self.current_token.r#type())
        {
            for op in [Op::Add, Op::Sub] {
                if !op.equal_type(self.current_token.r#type()) {
                    continue;
                }
                self.eat(op.r#type());
                let node: Rc<dyn Node> = match self.factor() {
                    Ok(n) => n,
                    Err(e) => return Err(e),
                };
                return Ok(Rc::new(UnaryOp::new(op.r#type(), node)));
            }
            println!(
                "[parser] [factor] current token {} is invalid, '+' or '-' operation is required",
                self.current_token
            );
            Err(Error::InvalidSyntax)
        } else if self.current_token.r#type() == ID {
            Ok(self.variable())
        } else {
            println!(
                "[parser] [factor] current token {} is invalid",
                self.current_token
            );
            Err(Error::InvalidSyntax)
        }
    }

    /// BNF:
    /// term: factor ((MUL | DIV) factor)*
    fn term(&mut self) -> Result<Rc<dyn Node>, Error> {
        let node = match self.factor() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let mut result = node;
        while Op::Mul.equal_type(self.current_token.r#type())
            || Op::Div.equal_type(self.current_token.r#type())
        {
            for op in [Op::Mul, Op::Div] {
                if !op.equal_type(self.current_token.r#type()) {
                    continue;
                }
                self.eat(op.r#type());
                match self.factor() {
                    Ok(n) => {
                        let bin_op =
                            BinOp::new(result.clone(), Token::new(op.r#type(), op.value()), n);
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(result)
    }

    /// BNF:
    /// expr: term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> Result<Rc<dyn Node>, Error> {
        let node = match self.term() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let mut result = node;

        while Op::Add.equal_type(self.current_token.r#type())
            || Op::Sub.equal_type(self.current_token.r#type())
        {
            for op in [Op::Add, Op::Sub] {
                if !op.equal_type(self.current_token.r#type()) {
                    continue;
                }
                self.eat(op.r#type());
                match self.term() {
                    Ok(n) => {
                        let bin_op =
                            BinOp::new(result.clone(), Token::new(op.r#type(), op.value()), n);
                        result = Rc::new(bin_op)
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(result)
    }
}
