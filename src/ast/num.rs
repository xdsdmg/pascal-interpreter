/// num.rs implements the AST node of Num type.
use super::{Node, NodeType};
use crate::error::Error;
use crate::token::Token;

pub struct Num {
    token: Token,
    value: i32,
}

impl Num {
    pub fn new(token: Token, value: i32) -> Num {
        Num { token, value }
    }
}

impl Node for Num {
    fn get_type(&self) -> NodeType {
        NodeType::Num
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        Ok(Some(self.token.value().to_string()))
    }
}
