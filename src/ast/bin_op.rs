use super::{Info, Node, NodeType, Value};
use crate::error::Error;
use crate::lexer::lexeme::number::NumberType;
use crate::lexer::lexeme::{op::Op, Type};
use crate::token::Token;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use std::str::FromStr;

pub struct BinOp {
    left: Rc<dyn Node>,
    token: Token,
    right: Rc<dyn Node>,
}

impl BinOp {
    pub fn new(left: Rc<dyn Node>, token: Token, right: Rc<dyn Node>) -> BinOp {
        BinOp { left, token, right }
    }
}

fn cal<T>(left: &str, r#type: &str, right: &str) -> Result<T, Error>
where
    T: FromStr
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::fmt::Display
        + Copy,
    <T as FromStr>::Err: std::fmt::Display,
{
    let mut nums: Vec<T> = Vec::new();
    for n in [left, right] {
        match n.parse::<T>() {
            Ok(num) => nums.push(num),
            Err(e) => {
                println!("[visit] [BinOp] parse num {} failed, error: {}", left, e);
                return Err(Error::InvalidSyntax);
            }
        };
    }

    let r: T;
    if Op::Sub.equal_type(r#type) {
        r = nums[0] - nums[1];
    } else if Op::Add.equal_type(r#type) {
        r = nums[0] + nums[1];
    } else if Op::Mul.equal_type(r#type) {
        r = nums[0] * nums[1];
    } else if Op::Div.equal_type(r#type) {
        r = nums[0] / nums[1];
    } else {
        println!("[visit] [BinOp] invalid op {}", r#type);
        return Result::Err(Error::InvalidSyntax);
    }

    Ok(r)
}

impl Node for BinOp {
    fn r#type(&self) -> NodeType {
        NodeType::BinOp
    }

    fn visit(&self) -> Result<Info, Error> {
        let mut vals: Vec<Value> = Vec::new();
        for n in [self.left.clone(), self.right.clone()] {
            match n.visit() {
                Ok(info) => match info.value() {
                    Some(v) => vals.push(v),
                    None => {
                        println!(
                            "[visit] [{}] value not found in this node",
                            self.r#type().as_str()
                        );
                        return Err(Error::InvalidSyntax);
                    }
                },
                Err(e) => return Err(e),
            };
        }

        let val: Value;
        if NumberType::Real.equal_type(&vals[0].r#type)
            || NumberType::Real.equal_type(&vals[1].r#type)
        {
            let r = match cal::<f32>(&vals[0].value, self.token.r#type(), &vals[1].value) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            val = Value::new(&r.to_string(), NumberType::Real.r#type())
        } else {
            let r = match cal::<i32>(&vals[0].value, self.token.r#type(), &vals[1].value) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            val = Value::new(&r.to_string(), NumberType::Integer.r#type())
        }

        Ok(Info::new(None, NodeType::BinOp, Some(val)))
    }
}
