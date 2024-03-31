use super::{Info, NodeType};
use crate::global_scope::Scope;
use crate::{ast::Node, error::Error};
use std::{cell::RefCell, rc::Rc};

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

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        Ok(Info::new(None, NodeType::NoOp, None))
    }
}
