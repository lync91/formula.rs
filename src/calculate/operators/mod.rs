pub mod boolean;
pub mod comparison;
pub mod date;
pub mod numeric;
pub mod string;
pub mod values;

use super::functions::calculate_function;
use crate::error::ValueError;
use crate::parser::operators::Operator;
use crate::value::Value;
use crate::Expression;
pub use boolean::*;
pub use comparison::*;
pub use date::*;
pub use numeric::*;
pub use string::*;
pub use values::*;
pub fn calculate_operation(exp: Expression, f: Option<&impl Fn(&str) -> Value>) -> Value {
    match exp.op {
        Operator::Plus => {
            let (value2, value1) = get_values(exp, f);
            match value1 {
                Value::Date(d) => add_days_to_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2),
            }
        }

        Operator::Minus => {
            let (value2, value1) = get_values(exp, f);
            match value1 {
                Value::Date(d) => subtract_days_from_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2),
            }
        }

        Operator::Multiply => {
            let (value2, value1) = get_values(exp, f);
            calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
        }
        Operator::Divide => {
            let (value2, value1) = get_values(exp, f);
            match value2 {
                Value::Number(x) if x == 0.0 => Value::Error(ValueError::Div0),
                _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
            }
        }
        Operator::Power => {
            let (value2, value1) = get_values(exp, f);
            calculate_numeric_operator(value1, value2, calculate_power_operator)
        }
        Operator::Concat => {
            let (value2, value1) = get_values(exp, f);
            calculate_string_operator(value1, value2, calculate_concat_operator)
        }
        Operator::Equal => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 == d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| (n1 - n2).abs() == 0.0),
            }
        }
        Operator::NotEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 != d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| (n1 - n2).abs() > 0.0),
            }
        }
        Operator::Greater => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 > d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 > n2),
            }
        }
        Operator::Less => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 < d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 < n2),
            }
        }
        Operator::GreaterOrEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 >= d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 >= n2),
            }
        }
        Operator::LessOrEqual => {
            let (value2, value1) = get_values(exp, f);
            match (value1.clone(), value2.clone()) {
                (Value::Date(l), Value::Date(r)) => compare_dates(l, r, |d1, d2| d1 <= d2),
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 <= n2),
            }
        }
        Operator::Function(func) => calculate_function(func, exp, f),
    }
}
