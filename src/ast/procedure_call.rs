use super::{Info, NodeType};
use crate::error::Error;
use crate::global_scope::{Scope, VariableSymbol};
use crate::lexer::lexeme::number::NumberType;
use crate::{ast::Node, global_scope::Identifier};
use std::{cell::RefCell, rc::Rc};

pub struct ProcedureCall {
    name: String,
    parameters: Vec<Rc<dyn Node>>,
}

impl ProcedureCall {
    pub fn new(id: &str, parameters: Vec<Rc<dyn Node>>) -> Self {
        Self {
            name: id.to_string(),
            parameters,
        }
    }
}

impl Node for ProcedureCall {
    fn r#type(&self) -> NodeType {
        NodeType::ProcedureCall
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        let new_scope = Scope::new(&self.name, Some(scope.clone()), scope.borrow().level() + 1);
        let new_scope = Rc::new(RefCell::new(new_scope));

        let id = match new_scope.borrow().get(&self.name) {
            Some(id) => id,
            None => {
                println!(
                    "[visit] [ProcedureCall] procedure '{}' not found in scope",
                    self.name
                );
                return Err(Error::ProcedureNotFound);
            }
        };

        let ps = match id {
            Identifier::Procedure(ps) => ps,
            _ => return Err(Error::InvalidSyntax),
        };

        /* Set the parameters of procedure */
        let mut params = self.parameters.iter();
        for vd in ps.procedure().var_decl_list() {
            let info = match vd.visit(new_scope.clone()) {
                Ok(info) => info,
                Err(e) => return Err(e),
            };

            let vd_val = match info.value() {
                Some(v) => v,
                None => {
                    println!(
                        "[visit] [ProcedureCall] the value of variable symbol '{}' is none",
                        info
                    );
                    return Err(Error::InvalidSyntax);
                }
            };

            for s in vd_val.value().split(",") {
                let param = match params.next() {
                    Some(p) => p,
                    None => return Err(Error::InvalidSyntax),
                };

                let info = match param.visit(new_scope.clone()) {
                    Ok(info) => info,
                    Err(e) => return Err(e),
                };

                let param_val = match info.value() {
                    Some(v) => v,
                    None => return Err(Error::InvalidSyntax),
                };

                let r#type = match NumberType::to_number_type(param_val.r#type()) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let vs = Identifier::Variable(VariableSymbol::new(
                    r#type,
                    Some(param_val.value().to_string()),
                ));
                if let Err(e) = new_scope.borrow_mut().set(s, vs) {
                    return Err(e);
                }
            }
        }

        let r = match ps.procedure().block().visit(new_scope.clone()) {
            Ok(info) => Ok(Info::new(None, NodeType::ProcedureCall, info.value)),
            Err(e) => Err(e),
        };

        new_scope.borrow().print();

        r
    }
}
