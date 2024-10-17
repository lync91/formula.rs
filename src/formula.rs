use crate::{parser::operators::Operator, value::Value};

#[derive(Debug)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(String),
    Iterator(Vec<Formula>),
}

#[derive(Debug)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

pub type NoReference<'a> = &'a fn(&str) -> Value;
pub type NoCustomFunction<'a> = &'a fn(&str, Vec<f64>) -> Value;
