use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::{self, Identifier};

pub struct VarDecl {
    ids: Vec<String>,
    r#type: String,
}

impl VarDecl {
    pub fn new(ids: Vec<String>, r#type: &str) -> VarDecl {
        VarDecl {
            ids,
            r#type: r#type.to_string(),
        }
    }
}

impl Node for VarDecl {
    fn r#type(&self) -> NodeType {
        NodeType::VarDecl
    }

    fn visit(&self) -> Result<Info, Error> {
        for key in self.ids.iter() {
            let identifier = Identifier::new(&self.r#type, None);
            global_scope::global_scope_set(&key, &identifier);
        }

        Ok(Info::new(
            None,
            self.r#type(),
            Some(Value::new("", &self.r#type)),
        ))
    }
}
