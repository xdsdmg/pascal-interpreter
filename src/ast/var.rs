/// var.rs implements the AST node of Var type.
use super::{Node, NodeType};
use crate::{error::Error, global_scope::global_scope_get, token::Token};

pub struct Var {
    token: Token,
    value: String,
    r#type: String,
}

impl Var {
    pub fn new(token: &Token) -> Var {
        Var {
            token: Token::new(token.r#type(), token.value()),
            value: token.value().to_string(),
        }
    }
}

impl Node for Var {
    fn get_type(&self) -> NodeType {
        NodeType::Var
    }

    fn visit(&self) -> Result<Option<String>, Error> {
        match global_scope_get(&self.value) {
            Some(v) => Ok(Some(v.clone())),
            None => Err(Error::VarNotFound),
        }
    }

    fn optional(&self) -> String {
        self.value.clone()
    }
}
