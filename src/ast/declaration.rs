use super::{procedure::Procedure, var_decl::VarDecl, Info, Node, NodeType};
use crate::error::Error;
use std::rc::Rc;

pub struct Declaration {
    var_decl_list: Vec<Rc<VarDecl>>,
    procedure_list: Vec<Rc<Procedure>>,
}

impl Declaration {
    pub fn new(var_decl_list: Vec<Rc<VarDecl>>, procedure_list: Vec<Rc<Procedure>>) -> Declaration {
        Declaration {
            var_decl_list,
            procedure_list,
        }
    }

    pub fn var_decl_list_push(&mut self, vd: Rc<VarDecl>) {
        self.var_decl_list.push(vd.clone());
    }

    pub fn procedure_list_push(&mut self, procedure: Rc<Procedure>) {
        self.procedure_list.push(procedure.clone());
    }
}

impl Node for Declaration {
    fn r#type(&self) -> NodeType {
        NodeType::Declaration
    }

    fn visit(&self) -> Result<Info, Error> {
        self.var_decl_list.iter().for_each(|vd| {
            let _ = vd.visit();
        });
        Ok(Info::new(None, self.r#type(), None))
    }
}
