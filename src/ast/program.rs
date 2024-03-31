use super::block::Block;
use super::{Info, NodeType};
use crate::ast::Node;
use crate::error::Error;
use crate::global_scope::Scope;
use std::{cell::RefCell, rc::Rc};

pub struct Program {
    name: String,
    block: Block,
}

impl Program {
    pub fn new(name: &str, block: Block) -> Program {
        Program {
            name: name.to_string(),
            block,
        }
    }
}

impl Node for Program {
    fn r#type(&self) -> NodeType {
        NodeType::Program
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        let new_scope = Scope::new(&self.name, Some(scope.clone()), scope.borrow().level() + 1);
        let new_scope = Rc::new(RefCell::new(new_scope));

        let val = match self.block.visit(new_scope.clone()) {
            Ok(info) => info.value,
            Err(e) => return Err(e),
        };

        new_scope.borrow().print();

        Ok(Info::new(Some(self.name.clone()), self.r#type(), val))
    }
}
