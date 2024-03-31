use super::{procedure::Procedure, var_decl::VarDecl, Info, Node, NodeType};
use crate::error::Error;
use crate::global_scope::Scope;
use std::{cell::RefCell, rc::Rc};

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

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        for vd in self.var_decl_list.iter() {
            if let Err(e) = vd.visit(scope.clone()) {
                return Err(e);
            }
        }
        Ok(Info::new(None, self.r#type(), None))
    }
}
