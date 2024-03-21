/// assign.rs implements the AST node of Assign type.
use super::{Node, NodeType};
use crate::{error::Error, global_scope::global_scope_set, token::Token};
use std::rc::Rc;

pub struct Assign {
    left: String,
    token: Token,
    right: Rc<dyn Node>,
}

impl Assign {
    pub fn new(left: &str, token: &Token, right: Rc<dyn Node>) -> Assign {
        Assign {
            left: left.to_string(),
            token: token.clone(),
            right,
        }
    }
}

impl Node for Assign {
    fn get_type(&self) -> NodeType {
        NodeType::Assign
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        match self.right.visit() {
            Ok(v) => global_scope_set(&self.left, &v.unwrap_or(String::from(""))),
            Err(e) => return Err(e),
        };

        Ok(None)
    }
}
