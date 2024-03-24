use super::{Info, Node, NodeType, Value};
use crate::{error::Error, global_scope::global_scope_get};

pub struct Var {
    value: String,
}

impl Var {
    pub fn new(value: &str) -> Var {
        Var {
            value: value.to_string(),
        }
    }
}

impl Node for Var {
    fn r#type(&self) -> NodeType {
        NodeType::Var
    }

    fn visit(&self) -> Result<Info, Error> {
        let value: Option<Value> = match global_scope_get(&self.value) {
            Some(identifier) => Some(Value::new(
                &identifier.value().unwrap_or(String::from("")),
                identifier.r#type(),
            )),
            None => None,
        };

        Ok(Info::new(Some(self.value.clone()), NodeType::Var, value))
    }
}
