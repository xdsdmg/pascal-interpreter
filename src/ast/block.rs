use super::{compound::Compound, declaration::Declaration, Info, Node, NodeType};
use crate::error::Error;
use crate::global_scope::Scope;
use std::{cell::RefCell, rc::Rc};

pub struct Block {
    declaration: Declaration,
    compound: Compound,
}

impl Block {
    pub fn new(declaration: Declaration, compound: Compound) -> Block {
        Block {
            declaration,
            compound,
        }
    }
}

impl Node for Block {
    fn r#type(&self) -> NodeType {
        NodeType::Block
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        if let Err(e) = self.declaration.visit(scope.clone()) {
            return Err(e);
        }
        let result = match self.compound.visit(scope.clone()) {
            Ok(info) => Ok(Info::new(None, self.r#type(), info.value())),
            Err(e) => Err(e),
        };
        result
    }
}
