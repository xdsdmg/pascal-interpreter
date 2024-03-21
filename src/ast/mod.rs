/// ast mod implements the AST node of our Pascal interpreter.
use crate::error::Error;

pub mod assign;
pub mod bin_op;
pub mod block;
pub mod compound;
pub mod no_op;
pub mod num;
pub mod program;
pub mod unary_op;
pub mod var;
pub mod var_decl;

pub trait Node {
    fn get_type(&self) -> NodeType;

    fn visit(&self) -> Result<Option<String>, Error>;

    fn optional(&self) -> String {
        String::from("")
    }
}

pub enum NodeType {
    Assign,
    BinOp,
    Compound,
    NoOp,
    Var,
    Num,
    UnaryOp,
    Program,
    Block,
}
