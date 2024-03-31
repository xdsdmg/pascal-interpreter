use super::{Info, Node, NodeType, Value};
use crate::global_scope::Scope;
use crate::{error::Error, global_scope::global_scope_get};
use std::{cell::RefCell, rc::Rc};

pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Var {
        Var {
            name: name.to_string(),
        }
    }
}

impl Node for Var {
    fn r#type(&self) -> NodeType {
        NodeType::Var
    }

    fn name(&self) -> Result<Option<String>, Error> {
        return Ok(Some(self.name.clone()));
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        let value: Option<Value> = match scope.borrow().get(&self.name) {
            Some(identifier) => Some(Value::new(
                &identifier.value().unwrap_or(String::from("")),
                identifier.r#type(),
            )),
            None => None,
        };

        Ok(Info::new(Some(self.name.clone()), NodeType::Var, value))
    }
}
