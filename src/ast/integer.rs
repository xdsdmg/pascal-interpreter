use super::{Info, Node, NodeType, Value};
use crate::{
    error::Error,
    lexer::lexeme::{number::NumberType, Type},
};

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

    fn visit(&self) -> Result<Info, Error> {
        Ok(Info::new(
            None,
            NodeType::Integer,
            Some(Value::new(
                &self.value.to_string(),
                NumberType::Integer.r#type(),
            )),
        ))
    }
}
