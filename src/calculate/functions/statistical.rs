use error::ValueError;

use crate::calculate::{
    calculate_formula,
    operators::{
        calcualte_numeric_operator_rhs_text, calculate_divide_operator, calculate_numeric_operator,
    },
};
use crate::value::Value;
use crate::*;
pub fn calculate_average(
    mut collective_value: Value,
    mut exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    let mut element_count = 0;
    while let Some(top) = exp.values.pop() {
        element_count += 1;
        collective_value = calculate_average_operator(
            &mut element_count,
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    if element_count == 0 {
        Value::Error(ValueError::Div0)
    } else {
        calculate_numeric_operator(
            collective_value,
            Value::Number(element_count as f64),
            calculate_divide_operator,
        )
    }
}

fn calculate_average_operator_rhs_number(
    element_count: &mut i32,
    l: f64,
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match rhs {
        Value::Boolean(_) => rhs,
        Value::Error(_) => rhs,
        Value::Text(t) => match t.parse::<f64>() {
            Ok(nr) => Value::Number(f(l, nr)),
            Err(_) => Value::Error(ValueError::Cast),
        },
        Value::Number(r) => Value::Number(f(l, r)),
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                match temp {
                    Value::Blank => *element_count -= 1,
                    _ => (),
                };
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top.clone(), f);
                    match top {
                        Value::Blank => (),
                        _ => *element_count += 1,
                    };
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => {
            *element_count -= 1;
            Value::Number(f(l, 0.0))
        }
    }
}

fn calculate_average_operator_rhs_iterator(
    element_count: &mut i32,
    mut lhs_vec: Vec<Value>,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match rhs {
        Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                    *element_count += 1;
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        _ => Value::Error(ValueError::Value),
    }
}

fn calculate_average_operator(
    element_count: &mut i32,
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Value {
    match lhs {
        Value::Boolean(_) => lhs,
        Value::Error(_) => lhs,
        Value::Text(t) => calcualte_numeric_operator_rhs_text(t, rhs, f),
        Value::Number(l) => calculate_average_operator_rhs_number(element_count, l, lhs, rhs, f),
        Value::Iterator(lhs_vec) => {
            calculate_average_operator_rhs_iterator(element_count, lhs_vec, rhs, f)
        }
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => {
            *element_count -= 1;
            calculate_average_operator_rhs_number(element_count, 0.0, lhs, rhs, f)
        }
    }
}
