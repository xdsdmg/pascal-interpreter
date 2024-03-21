use std::rc::Rc;

use super::{compound::Compound, var_decl::VarDecl};

pub struct Block {
    var_decl_list: Vec<Rc<VarDecl>>,
    compound: Compound,
}
