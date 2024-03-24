use super::{compound::Compound, var_decl::VarDecl, Info, Node, NodeType};
use crate::error::Error;
use std::rc::Rc;

pub struct Block {
    var_decl_list: Vec<Rc<VarDecl>>,
    compound: Compound,
}

impl Block {
    pub fn new(var_decl_list: Vec<Rc<VarDecl>>, compound: Compound) -> Block {
        Block {
            var_decl_list,
            compound,
        }
    }
}

impl Node for Block {
    fn r#type(&self) -> NodeType {
        NodeType::Block
    }

    fn visit(&self) -> Result<Info, Error> {
        for vd in self.var_decl_list.iter() {
            let _ = vd.visit();
        }
        let result = match self.compound.visit() {
            Ok(info) => Ok(Info::new(None, NodeType::Block, info.value())),
            Err(e) => Err(e),
        };
        result
    }
}
