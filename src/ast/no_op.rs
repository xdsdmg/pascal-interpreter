/// no_op.rs implements the AST node of NoOp type.
use super::NodeType;
use crate::{ast::Node, error::Error};

pub struct NoOp {}

impl NoOp {
    pub fn new() -> NoOp {
        NoOp {}
    }
}

impl Node for NoOp {
    fn get_type(&self) -> NodeType {
        NodeType::NoOp
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        Ok(None)
    }
}
