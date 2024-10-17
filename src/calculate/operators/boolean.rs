use crate::{error::ValueError, value::Boolean, Value};

pub fn to_bool(value: Boolean) -> bool {
    match value {
        Boolean::True => true,
        Boolean::False => false,
    }
}

pub fn calculate_boolean_operator(
    lhs: Value,
    rhs: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        Value::Error(_) => calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs)),
        Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        Value::Blank => {
            calculate_boolean_operator_rhs_boolean(Boolean::True, cast_value_to_boolean(rhs), f)
        }
        _ => Value::Error(ValueError::Value),
    }
}

pub fn calculate_boolean_operator_rhs_boolean(
    l: Boolean,
    rh: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    match rh {
        Value::Boolean(r) => {
            if f(to_bool(l), to_bool(r)) {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Error(_) => match l {
            Boolean::True => Value::Boolean(Boolean::True),
            Boolean::False => Value::Boolean(Boolean::False),
        },
        Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                let rhs = cast_value_to_boolean(temp);
                match rhs {
                    Value::Boolean(r) => {
                        if f(to_bool(l), to_bool(r)) {
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
        Value::Blank => {
            if to_bool(l) {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        _ => Value::Error(ValueError::Value),
    }
}

pub fn calculate_boolean_operator_rhs_error(rh: Value) -> Value {
    match rh {
        Value::Boolean(r) => {
            if to_bool(r) {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Error(_) => Value::Error(ValueError::Cast),
        _ => Value::Error(ValueError::Value),
    }
}

pub fn calculate_boolean_operator_rhs_iterator(
    rh: Value,
    mut lhs_vec: Vec<Value>,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    match rh {
        Value::Boolean(r) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f);
                }
                let lhs = cast_value_to_boolean(temp);
                match lhs {
                    Value::Boolean(l) => {
                        if f(to_bool(l), to_bool(r)) {
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

        _ => Value::Error(ValueError::Value),
    }
}

pub fn calculate_boolean_operator_or(
    lhs: Value,
    rhs: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        Value::Error(_) => calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs)),
        Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        Value::Blank => {
            calculate_boolean_operator_rhs_boolean(Boolean::False, cast_value_to_boolean(rhs), f)
        }
        _ => Value::Error(ValueError::Value),
    }
}

pub fn calculate_boolean_operator_xor(
    lhs: Value,
    rhs: Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Value {
    let lh = cast_value_to_boolean(lhs);
    match lh {
        Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f)
        }
        Value::Error(_) => calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs)),
        Value::Iterator(lhs_vec) => {
            calculate_boolean_operator_rhs_iterator(cast_value_to_boolean(rhs), lhs_vec, f)
        }
        Value::Blank => {
            calculate_boolean_operator_rhs_boolean(Boolean::False, cast_value_to_boolean(rhs), f)
        }
        _ => Value::Error(ValueError::Value),
    }
}

pub fn cast_text_to_boolean(s: &str) -> Option<Boolean> {
    if s.eq_ignore_ascii_case("TRUE") {
        Some(Boolean::True)
    } else if s.eq_ignore_ascii_case("FALSE") {
        Some(Boolean::False)
    } else {
        None
    }
}

pub fn cast_value_to_boolean(value: Value) -> Value {
    match value {
        Value::Boolean(_) => value,
        Value::Error(_) => value,
        Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => {
                    if to_bool(l) {
                        Value::Boolean(Boolean::True)
                    } else {
                        Value::Boolean(Boolean::False)
                    }
                }
                None => Value::Error(ValueError::Cast),
            }
        }
        Value::Number(l) => {
            if l != 0.0 {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Iterator(mut value_vec) => {
            let mut boolean_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                let value = cast_value_to_boolean(top);
                boolean_vec.push(value);
            }
            Value::Iterator(boolean_vec)
        }
        Value::Date(_) => Value::Error(ValueError::Cast),
        Value::Blank => Value::Blank,
    }
}

pub fn calculate_negation(value: Value) -> Value {
    match value {
        Value::Boolean(l) => {
            if !(to_bool(l)) {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Error(_) => value,
        Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => {
                    if !(to_bool(l)) {
                        Value::Boolean(Boolean::True)
                    } else {
                        Value::Boolean(Boolean::False)
                    }
                }
                None => Value::Error(ValueError::Cast),
            }
        }
        Value::Number(l) => {
            if l == 0.0 {
                Value::Boolean(Boolean::True)
            } else {
                Value::Boolean(Boolean::False)
            }
        }
        Value::Iterator(_) => Value::Error(ValueError::Value),
        Value::Date(_) => Value::Error(ValueError::Value),
        Value::Blank => Value::Boolean(Boolean::True),
    }
}
