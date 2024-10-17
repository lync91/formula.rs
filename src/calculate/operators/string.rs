use crate::{error::ValueError, Value};

pub fn calculate_concat_operator(str1: &str, str2: &str) -> String {
    str1.to_owned() + str2
}

pub fn calculate_string_operator(
    lhs: Value,
    rhs: Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> Value {
    match lhs {
        Value::Boolean(_) => lhs,
        Value::Error(_) => lhs,
        Value::Number(l) => calculate_string_operation_rhs(&l.to_string(), rhs, f),
        Value::Text(l) => calculate_string_operation_rhs(&l, rhs, f),
        Value::Iterator(_) => Value::Error(ValueError::Value),
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => calculate_string_operation_rhs("", rhs, f),
    }
}

pub fn calculate_string_operation_rhs(
    l: &str,
    rhs: Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> Value {
    match rhs {
        Value::Boolean(_) => rhs,
        Value::Error(_) => rhs,
        Value::Number(r) => Value::Text(f(&l, &r.to_string())),
        Value::Text(r) => Value::Text(f(&l, &r)),
        Value::Iterator(_) => Value::Error(ValueError::Value),
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => Value::Text(f(&l, "")),
    }
}
