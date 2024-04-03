use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::Scope;
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::Type;
use std::{cell::RefCell, rc::Rc};

pub struct Real {
    value: f32,
}

impl Real {
    pub fn new(value: f32) -> Real {
        Real { value }
    }
}

impl Node for Real {
    fn r#type(&self) -> NodeType {
        NodeType::Real
    }

    fn visit(&self, _scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        Ok(Info::new(
            None,
            NodeType::Real,
            Some(Value::new(
                NumberType::Real.r#type(),
                &self.value.to_string(),
            )),
        ))
    }
}
