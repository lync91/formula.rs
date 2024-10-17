use super::calculator::calculate_formula;
use crate::error::ValueError;
use crate::parser;
use crate::value::Value;
use crate::NoCustomFunction;

pub fn calculate_reference(r: &str, f: Option<&impl Fn(&str) -> Value>) -> Value {
    match f {
        Some(f) => match f(r) {
            Value::Number(x) => Value::Number(x),
            Value::Text(s) => {
                calculate_formula(parser::parse_formula(&s, None::<NoCustomFunction>), Some(f))
            }
            Value::Boolean(x) => Value::Boolean(x),
            Value::Error(ValueError::Value) => Value::Error(ValueError::Value),
            Value::Iterator(v) => Value::Iterator(v),
            Value::Date(d) => Value::Date(d),
            Value::Blank => Value::Blank,
            _ => Value::Error(ValueError::Reference),
        },
        None => Value::Error(ValueError::Reference),
    }
}
