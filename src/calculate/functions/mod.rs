pub mod date;
pub mod logical;
pub mod math;
pub mod statistical;
pub mod text;

use crate::{parser::function::Function, value::Value, Expression};

use super::operators::*;
pub use date::*;
pub use logical::*;
pub use math::*;
pub use statistical::*;
pub use text::*;

pub fn calculate_function(
    func: Function,
    exp: Expression,
    f: Option<&impl Fn(&str) -> Value>,
) -> Value {
    match func {
        Function::Abs => calculate_abs(get_value(exp, f)),
        Function::Sum => {
            calculate_collective_operator(Value::Number(0.0), exp, f, |n1, n2| n1 + n2)
        }
        Function::Product => {
            calculate_collective_product_operator(Value::Blank, exp, f, |n1, n2| n1 * n2)
        }
        Function::Average => calculate_average(Value::Number(0.00), exp, f, |n1, n2| n1 + n2),
        Function::Or => calculate_or(exp, f, |n1, n2| n1 || n2),
        Function::And => calculate_bool(exp, f, |n1, n2| n1 && n2),
        Function::Xor => calculate_xor(exp, f, |n1, n2| n1 ^ n2),
        Function::Not => calculate_negation(get_value(exp, f)),
        Function::Negate => calculate_negate(get_value(exp, f)),
        Function::Days => calculate_days(get_date_values(exp, f)),
        Function::Right => calculate_right(get_number_and_string_values(exp, f)),
        Function::Left => calculate_left(get_number_and_string_values(exp, f)),
        Function::Iff => calculate_iff(get_iff_values(exp, f)),
        Function::IsBlank => calculate_isblank(get_value(exp, f)),
    }
}
