use super::{Info, NodeType};
use crate::ast::Node;
use crate::global_scope::Scope;
use crate::error::Error;
use std::{cell::RefCell, rc::Rc};

pub struct ProcedureCall {
    id: String,
    parameters: Vec<String>,
}

impl ProcedureCall {
    pub fn new(id: &str, parameters: Vec<String>) -> ProcedureCall {
        ProcedureCall {
            id: id.to_string(),
            parameters,
        }
    }
}

impl Node for ProcedureCall {
    fn r#type(&self) -> NodeType {
        NodeType::ProcedureCall
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        todo!()
    }
}
