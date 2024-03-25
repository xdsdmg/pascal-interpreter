use super::{compound::Compound, declaration::Declaration, Info, Node, NodeType};
use crate::error::Error;

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

    fn visit(&self) -> Result<Info, Error> {
        let _ = self.declaration.visit(); // No error
        let result = match self.compound.visit() {
            Ok(info) => Ok(Info::new(None, self.r#type(), info.value())),
            Err(e) => Err(e),
        };
        result
    }
}
