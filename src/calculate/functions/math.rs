use super::super::calculator::calculate_formula;
use super::super::operators::*;
use crate::value::Value;
use crate::Expression;

pub fn calculate_abs(value: Value) -> Value {
    match value {
        Value::Number(l) => Value::Number(l.abs()),
        Value::Blank => Value::Number(0.0),
        _ => value,
    }
}

pub fn calculate_collective_operator(
    mut collective_value: Value,
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    while let Some(top) = exp.values.pop() {
        collective_value =
            calculate_numeric_operator(collective_value, calculate_formula(top, f), f_collective);
    }
    collective_value
}

pub fn calculate_collective_product_operator(
    mut collective_value: Value,
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    while let Some(top) = exp.values.pop() {
        collective_value = calculate_numeric_product_operator(
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    match collective_value {
        Value::Blank => Value::Number(0.0),
        _ => collective_value,
    }
}
// Implement other math functions here
