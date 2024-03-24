use super::block::Block;
use super::{Info, NodeType};
use crate::ast::Node;
use crate::error::Error;

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

    fn visit(&self) -> Result<Info, Error> {
        let val = match self.block.visit() {
            Ok(info) => info.value,
            Err(e) => return Err(e),
        };
        Ok(Info::new(Some(self.name.clone()), self.r#type(), val))
    }
}
