use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::Type;

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

    fn visit(&self) -> Result<Info, Error> {
        Ok(Info::new(
            None,
            NodeType::Real,
            Some(Value::new(
                &self.value.to_string(),
                NumberType::Real.r#type(),
            )),
        ))
    }
}
