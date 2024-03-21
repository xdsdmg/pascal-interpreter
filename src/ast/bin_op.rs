/// bin_op.rs implements the AST node of BinOp type.
use super::{Node, NodeType};
use crate::error::Error;
use crate::lexeme::{op::Op, Type};
use crate::token::Token;
use std::rc::Rc;

pub struct BinOp {
    left: Rc<dyn Node>,
    token: Token,
    right: Rc<dyn Node>,
}

impl BinOp {
    pub fn new(left: Rc<dyn Node>, token: Token, right: Rc<dyn Node>) -> BinOp {
        BinOp { left, token, right }
    }
}

impl Node for BinOp {
    fn get_type(&self) -> NodeType {
        NodeType::BinOp
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        let left_val = match self.left.visit() {
            Ok(v) => v
                .unwrap_or(String::from(""))
                .parse::<i32>()
                .expect(&Error::InvalidSyntax.to_string()),
            Err(e) => return Err(e),
        };

        let right_val = match self.right.visit() {
            Ok(v) => v
                .unwrap_or(String::from(""))
                .parse::<i32>()
                .expect(&Error::InvalidSyntax.to_string()),
            Err(e) => return Err(e),
        };

        let result: i32;

        if Op::Sub.equal_type(self.token.r#type()) {
            result = left_val - right_val;
        } else if Op::Add.equal_type(self.token.r#type()) {
            result = left_val + right_val;
        } else if Op::Mul.equal_type(self.token.r#type()) {
            result = left_val * right_val;
        } else if Op::Div.equal_type(self.token.r#type()) {
            result = left_val / right_val;
        } else {
            return Result::Err(Error::InvalidSyntax);
        }

        Ok(Some(result.to_string()))
    }
}
