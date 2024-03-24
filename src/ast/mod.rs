use crate::error::Error;
use std::fmt::{self, Display};

pub mod assign;
pub mod bin_op;
pub mod block;
pub mod compound;
pub mod integer;
pub mod no_op;
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
        }
    }
}

pub struct Info {
    name: Option<String>,
    r#type: NodeType,
    value: Option<Value>,
}

pub struct Value {
    value: String,
    r#type: String,
}

impl Value {
    pub fn new(value: &str, r#type: &str) -> Value {
        Value {
            value: value.to_string(),
            r#type: r#type.to_string(),
        }
    }

    pub fn clone(&self) -> Value {
        Value {
            value: self.value.clone(),
            r#type: self.r#type.clone(),
        }
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

pub trait Node {
    fn r#type(&self) -> NodeType {
        NodeType::Unknown
    }

    fn visit(&self) -> Result<Info, Error> {
        Err(Error::InvalidSyntax)
    }
}
