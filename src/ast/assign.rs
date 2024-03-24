use super::{Info, Node, NodeType};
use crate::{
    error::Error,
    global_scope::{global_scope_set, Identifier},
};
use std::rc::Rc;

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

    fn visit(&self) -> Result<Info, Error> {
        match self.right.visit() {
            Ok(info) => match info.value() {
                Some(v) => {
                    let identifier = Identifier::new(&v.r#type, Some(v.value));
                    global_scope_set(&self.left, &identifier);
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
