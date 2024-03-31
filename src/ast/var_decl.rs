use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::{Identifier, Scope};
use std::{cell::RefCell, rc::Rc};

pub struct VarDecl {
    ids: Vec<String>,
    r#type: String,
}

impl VarDecl {
    pub fn new(ids: Vec<String>, r#type: &str) -> VarDecl {
        VarDecl {
            ids,
            r#type: r#type.to_string(),
        }
    }
}

impl Node for VarDecl {
    fn r#type(&self) -> NodeType {
        NodeType::VarDecl
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        for key in self.ids.iter() {
            let identifier = Identifier::new(&self.r#type, None);
            if let Err(e) = scope.borrow_mut().set(key, &identifier) {
                return Err(e);
            }
        }

        Ok(Info::new(
            None,
            self.r#type(),
            Some(Value::new("", &self.r#type)),
        ))
    }
}
