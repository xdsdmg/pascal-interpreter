/// unary_op.rs implements the AST node of UnaryOp type.
use super::{Node, NodeType};
use crate::error::Error;
use crate::lexeme::{op::Op, Type};
use crate::token::Token;
use std::rc::Rc;

pub struct UnaryOp {
    token: Token,
    node: Rc<dyn Node>,
}

impl UnaryOp {
    pub fn new(token: Token, node: Rc<dyn Node>) -> UnaryOp {
        UnaryOp { token, node }
    }
}

impl Node for UnaryOp {
    fn get_type(&self) -> NodeType {
        NodeType::UnaryOp
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        let val = match self.node.visit() {
            Ok(v) => v
                .unwrap_or(String::from(""))
                .parse::<i32>()
                .expect(&Error::InvalidSyntax.to_string()),
            Err(e) => return Err(e),
        };

        let result: i32;
        if Op::Sub.equal_type(self.token.r#type()) {
            result = -val;
        } else if Op::Add.equal_type(self.token.r#type()) {
            result = val;
        } else {
            return Result::Err(Error::InvalidSyntax);
        }

        Ok(Some(result.to_string()))
    }
}
