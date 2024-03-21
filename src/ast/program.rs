use super::block::Block;
use crate::ast::Node;

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

impl Node for Program {}
