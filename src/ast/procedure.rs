use super::{block::Block, var_decl::VarDecl};
use std::rc::Rc;

pub struct Procedure {
    name: String,
    var_decl_list: Vec<Rc<VarDecl>>,
    block: Block,
}

impl Procedure {
    pub fn new(name: &str, var_decl_list: Vec<Rc<VarDecl>>, block: Block) -> Procedure {
        Procedure {
            name: name.to_string(),
            var_decl_list,
            block,
        }
    }
}
