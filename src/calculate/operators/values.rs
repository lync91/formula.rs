use super::super::calculator::calculate_formula;
use crate::error::ValueError;
use crate::value::Value;
use crate::Expression;
pub fn get_values(mut exp: Expression, f: Option<&impl Fn(&str) -> Value>) -> (Value, Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Error(ValueError::Argument),
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Error(ValueError::Argument),
        },
    )
}

pub fn get_value(mut exp: Expression, f: Option<&impl Fn(&str) -> Value>) -> Value {
    match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => Value::Error(ValueError::Argument),
    }
}

pub fn get_date_values(mut exp: Expression, f: Option<&impl Fn(&str) -> Value>) -> (Value, Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Error(ValueError::Argument),
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Error(ValueError::Argument),
        },
    )
}

pub fn get_number_and_string_values(
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
) -> (Value, Value) {
    if exp.values.len() == 1 {
        (
            Value::Number(1.0),
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => Value::Error(ValueError::Argument),
            },
        )
    } else {
        (
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => Value::Error(ValueError::Argument),
            },
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => Value::Error(ValueError::Argument),
            },
        )
    }
}

pub fn get_iff_values(
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
) -> (Value, Value, Value) {
    (
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Blank,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Blank,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => Value::Blank,
        },
    )
}
