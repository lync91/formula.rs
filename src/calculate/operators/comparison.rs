use crate::error::ValueError;
use crate::value::{Boolean, Value};
use chrono::{DateTime, FixedOffset};

pub fn calculate_comparison_operator(
    lhs: Value,
    rhs: Value,
    f: fn(num1: f64, num2: f64) -> bool,
) -> Value {
    match lhs {
        Value::Text(l) => match rhs {
            Value::Text(r) => {
                if l.eq(&r) {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            Value::Blank => {
                if l.is_empty() {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            _ => Value::Error(ValueError::Value),
        },
        Value::Number(l) => match rhs {
            Value::Number(r) => {
                if f(l, r) {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            Value::Blank => {
                if f(l, 0.0) {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            _ => Value::Error(ValueError::Value),
        },
        Value::Blank => match rhs {
            Value::Number(r) => {
                if f(0.0, r) {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            Value::Text(r) => {
                if r.is_empty() {
                    Value::Boolean(Boolean::True)
                } else {
                    Value::Boolean(Boolean::False)
                }
            }
            Value::Blank => Value::Boolean(Boolean::True),
            _ => Value::Error(ValueError::Value),
        },
        Value::Boolean(_) => Value::Error(ValueError::Value),
        Value::Error(_) => Value::Error(ValueError::Value),
        Value::Iterator(_) => Value::Error(ValueError::Value),
        Value::Date(_) => Value::Error(ValueError::Value),
    }
}

pub fn compare_dates(
    date1: DateTime<FixedOffset>,
    date2: DateTime<FixedOffset>,
    f: fn(d1: DateTime<FixedOffset>, d2: DateTime<FixedOffset>) -> bool,
) -> Value {
    if f(date1, date2) {
        Value::Boolean(Boolean::True)
    } else {
        Value::Boolean(Boolean::False)
    }
}
