use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::{Identifier, Scope};
use crate::lexer::lexeme::Type;
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
        let id = match scope.borrow().get(&self.name) {
            Some(id) => id,
            None => return Err(Error::VarNotFound),
        };

        let vs = match id {
            Identifier::Variable(vs) => vs,
            _ => return Err(Error::InvalidSyntax),
        };

        let value = match vs.value() {
            Some(v) => Some(Value::new(vs.r#type().r#type(), &v)),
            None => return Err(Error::InvalidSyntax),
        };

        Ok(Info::new(Some(self.name.clone()), NodeType::Var, value))
    }
}
