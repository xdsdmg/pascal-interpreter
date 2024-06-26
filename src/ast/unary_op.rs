use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::global_scope::Scope;
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::{op::Op, Type};
use std::ops::Neg;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

pub struct UnaryOp {
    r#type: String, // add or sub
    node: Rc<dyn Node>,
}

impl UnaryOp {
    pub fn new(r#type: &str, node: Rc<dyn Node>) -> UnaryOp {
        UnaryOp {
            r#type: r#type.to_string(),
            node,
        }
    }
}

fn cal<T>(val: &str, r#type: &str) -> Result<T, Error>
where
    T: FromStr + std::fmt::Display + Neg<Output = T>,
    <T as FromStr>::Err: std::fmt::Display,
{
    let r: T;
    let num = match val.parse::<T>() {
        Ok(n) => n,
        Err(e) => {
            println!("[visit] [UnaryOp] parse num '{}' failed, error: {}", val, e);
            return Err(Error::InvalidSyntax);
        }
    };

    if Op::Sub.equal_type(r#type) {
        r = -num;
    } else if Op::Add.equal_type(r#type) {
        r = num;
    } else {
        return Result::Err(Error::InvalidSyntax);
    }

    Ok(r)
}

impl Node for UnaryOp {
    fn r#type(&self) -> NodeType {
        NodeType::UnaryOp
    }

    fn visit(&self, scope: Rc<RefCell<Scope>>) -> Result<Info, Error> {
        let info = match self.node.visit(scope.clone()) {
            Ok(info) => match info.value {
                Some(val) => val,
                None => {
                    println!("[visit] val not found in UnaryOp");
                    return Err(Error::InvalidSyntax);
                }
            },
            Err(e) => return Err(e),
        };

        let val: Value;
        if NumberType::Real.equal_type(info.r#type) {
            let r = match cal::<f32>(&info.value, &self.r#type) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            val = Value::new(NumberType::Real.r#type(), &r.to_string());
        } else {
            let r = match cal::<i32>(&info.value, &self.r#type) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            val = Value::new(NumberType::Integer.r#type(), &r.to_string());
        }

        Ok(Info::new(None, NodeType::UnaryOp, Some(val)))
    }
}
