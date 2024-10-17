use super::operators::boolean::*;
use crate::{
    error::ValueError,
    value::{Boolean, Value},
};

pub fn convert_iterator_to_result(result: Value, f: fn(bool1: bool, bool2: bool) -> bool) -> Value {
    match result {
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            Value::Boolean(Boolean::True)
                        } else {
                            Value::Boolean(Boolean::False)
                        }
                    }
                    _ => Value::Error(ValueError::Value),
                }
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        _ => result,
    }
}

pub fn convert_iterator_to_result_or(
    result: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    match result {
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_or(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            Value::Boolean(Boolean::True)
                        } else {
                            Value::Boolean(Boolean::False)
                        }
                    }
                    _ => Value::Error(ValueError::Value),
                }
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        _ => result,
    }
}

pub fn convert_iterator_to_result_xor(
    result: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    match result {
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_xor(temp, top, f);
                }
                match cast_value_to_boolean(temp) {
                    Value::Boolean(bool_result) => {
                        if to_bool(bool_result) {
                            Value::Boolean(Boolean::True)
                        } else {
                            Value::Boolean(Boolean::False)
                        }
                    }
                    _ => Value::Error(ValueError::Value),
                }
            } else {
                Value::Error(ValueError::Argument)
            }
        }
        _ => result,
    }
}

/// Converts a result from Value Enum to a printable string.
pub fn result_to_string(value: Value) -> String {
    match value {
        Value::Number(number) => show_number(number),
        Value::Text(text) => text,
        Value::Error(error) => show_error(error),
        Value::Boolean(boolean) => show_boolean(boolean),
        Value::Iterator(value_vec) => show_iterator(value_vec),
        Value::Date(date) => date.to_string(),
        Value::Blank => show_blank(),
    }
}

fn show_number(number: f64) -> String {
    if number.is_infinite() {
        String::from("#DIV/0!")
    } else {
        number.to_string()
    }
}

fn show_error(error: ValueError) -> String {
    match error {
        ValueError::Div0 => String::from("#DIV/0!"),
        ValueError::Cast => String::from("#CAST!"),
        ValueError::Parse => String::from("#PARSE!"),
        ValueError::Value => String::from("#VALUE!"),
        ValueError::Argument => String::from("#ARG!"),
        ValueError::Reference => String::from("#REF!"),
    }
}

fn show_boolean(boolean: Boolean) -> String {
    match boolean {
        Boolean::True => String::from("TRUE"),
        Boolean::False => String::from("FALSE"),
    }
}

fn show_iterator(mut value_vec: Vec<Value>) -> String {
    value_vec.reverse();
    let mut result = '{'.to_string();
    while let Some(top) = value_vec.pop() {
        result = result + &result_to_string(top);
        result = result + &','.to_string();
    }
    result = result.trim_end_matches(',').to_string();
    result = result + &'}'.to_string();
    result
}

fn show_blank() -> String {
    show_number(0.0)
    //String::from("BLANK")
}
