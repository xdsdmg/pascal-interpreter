use super::Info;
use super::{Node, NodeType};
use crate::error::Error;
use crate::global_scope::Scope;
use std::{cell::RefCell, rc::Rc};

pub struct Compound {
    children: Vec<Rc<dyn Node>>,
}

impl Compound {
    pub fn new(children: Vec<Rc<dyn Node>>) -> Compound {
        Compound { children }
    }
}

impl Node for Compound {
    fn r#type(&self) -> NodeType {
        NodeType::Compound
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        for c in self.children.iter() {
            if let Err(e) = c.visit(scope.clone()) {
                return Err(e);
            }
        }
        Ok(Info::new(None, NodeType::Compound, None))
    }
}
