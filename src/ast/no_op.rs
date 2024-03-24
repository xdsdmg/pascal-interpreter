use super::{Info, NodeType};
use crate::{ast::Node, error::Error};

pub struct NoOp {}

impl NoOp {
    pub fn new() -> NoOp {
        NoOp {}
    }
}

impl Node for NoOp {
    fn r#type(&self) -> NodeType {
        NodeType::NoOp
    }

    fn visit(&self) -> Result<Info, Error> {
        Ok(Info::new(None, NodeType::NoOp, None))
    }
}
