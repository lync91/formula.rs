use super::super::calculator::calculate_formula;
use super::super::operators::boolean::*;
use super::super::utils::*;
use crate::error::ValueError;
use crate::value::{Boolean, Value};
use crate::Expression;

pub fn calculate_iff(iff_arguments: (Value, Value, Value)) -> Value {
    let (false_value, true_value, bool_expression) = iff_arguments;
    match bool_expression {
        Value::Boolean(bool_value) => {
            if to_bool(bool_value) {
                true_value
            } else {
                false_value
            }
        }
        Value::Number(number_value) => {
            if number_value == 0.0 {
                false_value
            } else {
                true_value
            }
        }
        Value::Blank => false_value,
        Value::Error(_) => bool_expression,
        _ => Value::Error(crate::error::ValueError::Value),
    }
}

pub fn calculate_isblank(value: Value) -> Value {
    match value {
        Value::Blank => Value::Boolean(Boolean::True),
        Value::Text(s) => {
            if s.is_empty() {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Error(ValueError::Value) => Value::Boolean(Boolean::True),
        Value::Error(ValueError::Reference) => Value::Boolean(Boolean::True),
        _ => Value::Boolean(Boolean::False),
    }
}

pub fn calculate_bool(
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => Value::Error(ValueError::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result(result, f_bool)
}

pub fn calculate_or(
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => Value::Error(ValueError::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_or(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result_or(result, f_bool)
}

pub fn calculate_xor(
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => Value::Error(ValueError::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_xor(result, calculate_formula(top, f), f_bool);
    }
    convert_iterator_to_result_xor(result, f_bool)
}
// Implement other logical functions here
