/// compound.rs implements the AST node of Compound type.
use super::{Node, NodeType};
use crate::error::Error;
use std::rc::Rc;

pub struct Compound {
    children: Vec<Rc<dyn Node>>,
}

impl Compound {
    pub fn new(children: Vec<Rc<dyn Node>>) -> Compound {
        Compound { children }
    }
}

impl Node for Compound {
    fn get_type(&self) -> NodeType {
        NodeType::Compound
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        for c in self.children.iter() {
            if let Err(e) = c.visit() {
                return Err(e);
            }
        }
        Ok(None)
    }
}
