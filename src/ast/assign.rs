use super::{Info, Node, NodeType};
use crate::{
    error::Error,
    global_scope::{Identifier, Scope, VariableSymbol},
    lexer::lexeme::number::NumberType,
};
use std::{cell::RefCell, rc::Rc};

pub struct Assign {
    left: String,
    right: Rc<dyn Node>,
}

impl Assign {
    pub fn new(left: &str, right: Rc<dyn Node>) -> Assign {
        Assign {
            left: left.to_string(),
            right,
        }
    }
}

impl Node for Assign {
    fn r#type(&self) -> NodeType {
        NodeType::Assign
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        match self.right.visit(scope.clone()) {
            Ok(info) => match info.value() {
                Some(v) => {
                    let r#type = match NumberType::to_number_type(&v.r#type) {
                        Ok(t) => t,
                        Err(e) => return Err(e),
                    };
                    if let Err(e) = scope.borrow_mut().set(
                        &self.left,
                        Identifier::Variable(VariableSymbol::new(r#type, Some(v.value))),
                    ) {
                        return Err(e);
                    }
                }
                None => {
                    println!(
                        "[visit] [{}] error occurred, value not found in info",
                        self.r#type().as_str()
                    );
                    return Err(Error::InvalidSyntax);
                }
            },
            Err(e) => return Err(e),
        };

        Ok(Info::new(None, NodeType::Assign, None))
    }
}
