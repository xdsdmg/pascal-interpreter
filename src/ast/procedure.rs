use super::{block::Block, var_decl::VarDecl};
use super::{Info, Node, NodeType};
use crate::global_scope::ProcedureSymbol;
use crate::{
    error::Error,
    global_scope::{Identifier, Scope},
};
use std::{cell::RefCell, rc::Rc};

pub struct Procedure {
    name: String,
    var_decl_list: Vec<Rc<VarDecl>>,
    block: Rc<Block>,
}

impl Clone for Procedure {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            var_decl_list: self.var_decl_list.clone(),
            block: self.block.clone(),
        }
    }
}

impl Procedure {
    pub fn new(name: &str, var_decl_list: Vec<Rc<VarDecl>>, block: Rc<Block>) -> Self {
        Procedure {
            name: name.to_string(),
            var_decl_list,
            block,
        }
    }

    pub fn var_decl_list(&self) -> Vec<Rc<VarDecl>> {
        self.var_decl_list.clone()
    }

    pub fn block(&self) -> Rc<Block> {
        self.block.clone()
    }
}

impl Node for Procedure {
    fn r#type(&self) -> NodeType {
        NodeType::Procedure
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        match scope.borrow_mut().define(
            &self.name,
            Identifier::Procedure(ProcedureSymbol::new(&self.name, Rc::new(self.clone()))),
        ) {
            Ok(_) => Ok(Info::new(None, NodeType::Procedure, None)),
            Err(e) => Err(e),
        }
    }
}
