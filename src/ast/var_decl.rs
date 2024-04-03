use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::{Identifier, Scope, VariableSymbol};
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::Type;
use std::{cell::RefCell, rc::Rc};

pub struct VarDecl {
    ids: Vec<String>,
    r#type: NumberType,
}

impl VarDecl {
    pub fn new(ids: Vec<String>, r#type: NumberType) -> VarDecl {
        VarDecl { ids, r#type }
    }
}

impl Node for VarDecl {
    fn r#type(&self) -> NodeType {
        NodeType::VarDecl
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        for key in self.ids.iter() {
            if let Err(e) = scope.borrow_mut().define(
                key,
                Identifier::Variable(VariableSymbol::new(self.r#type, None)),
            ) {
                return Err(e);
            }
        }

        Ok(Info::new(
            None,
            self.r#type(),
            Some(Value::new(self.r#type.r#type(), &self.ids.join(","))),
        ))
    }
}
