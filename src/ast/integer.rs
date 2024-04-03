use super::{Info, Node, NodeType, Value};
use crate::global_scope::Scope;
use crate::{
    error::Error,
    lexer::lexeme::{number::NumberType, Type},
};
use std::{cell::RefCell, rc::Rc};

pub struct Integer {
    value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Integer {
        Integer { value }
    }
}

impl Node for Integer {
    fn r#type(&self) -> NodeType {
        NodeType::Integer
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        Ok(Info::new(
            None,
            NodeType::Integer,
            Some(Value::new(
                NumberType::Integer.r#type(),
                &self.value.to_string(),
            )),
        ))
    }
}
