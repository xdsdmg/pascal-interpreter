use crate::{error::Error, global_scope::Scope};
use std::fmt::{self, Display};
use std::{cell::RefCell, rc::Rc};

pub mod assign;
pub mod bin_op;
pub mod block;
pub mod compound;
pub mod declaration;
pub mod integer;
pub mod no_op;
pub mod procedure;
pub mod procedure_call;
pub mod program;
pub mod real;
pub mod unary_op;
pub mod var;
pub mod var_decl;

pub enum NodeType {
    Unknown,
    Assign,
    BinOp,
    Compound,
    NoOp,
    Var,
    Integer,
    Real,
    UnaryOp,
    Program,
    Block,
    VarDecl,
    Declaration,
    ProcedureCall,
    Procedure,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Unknown => "Unknown",
            NodeType::Assign => "Assign",
            NodeType::BinOp => "BinOp",
            NodeType::Compound => "Compound",
            NodeType::NoOp => "NoOp",
            NodeType::Var => "Var",
            NodeType::Integer => "Integer",
            NodeType::Real => "Real",
            NodeType::UnaryOp => "UnaryOp",
            NodeType::Program => "Program",
            NodeType::Block => "Block",
            NodeType::VarDecl => "Variable Declaration",
            NodeType::Declaration => "Declaration",
            NodeType::ProcedureCall => "Procedure Call",
            NodeType::Procedure => "Procedure",
        }
    }
}

pub struct Info {
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    r#type: NodeType,
    value: Option<Value>,
}

pub struct Value {
    r#type: String,
    value: String,
}

impl Value {
    pub fn new(r#type: &str, value: &str) -> Value {
        Value {
            r#type: r#type.to_string(),
            value: value.to_string(),
        }
    }

    pub fn clone(&self) -> Value {
        Value {
            r#type: self.r#type.clone(),
            value: self.value.clone(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {}, value: {}", self.r#type, self.value)
    }
}

impl Info {
    pub fn new(name: Option<String>, r#type: NodeType, value: Option<Value>) -> Info {
        Info {
            name,
            r#type,
            value,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn value(&self) -> Option<Value> {
        if let Some(v) = &self.value {
            return Some(v.clone());
        }
        None
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match &self.name {
            Some(n) => n.clone(),
            None => String::from("none"),
        };

        match &self.value {
            Some(v) => write!(
                f,
                "{{name: {}, type: {}, value: {}}}",
                name,
                self.r#type.as_str(),
                v,
            ),
            None => write!(
                f,
                "{{name: {}, type: {}, value: none}}",
                name,
                self.r#type.as_str(),
            ),
        }
    }
}

pub trait Node {
    fn r#type(&self) -> NodeType {
        NodeType::Unknown
    }

    fn name(&self) -> Result<Option<String>, Error> {
        Err(Error::InvalidSyntax)
    }

    fn visit(&self, _scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        Err(Error::InvalidSyntax)
    }
}
