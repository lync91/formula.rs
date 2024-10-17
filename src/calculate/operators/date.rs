use crate::value::Value;
use chrono::{DateTime, Duration, FixedOffset};

pub fn add_days_to_date(d: DateTime<FixedOffset>, rhs: Value) -> Value {
    match rhs {
        Value::Number(x) => Value::Date(d + Duration::days(x as i64)),
        _ => Value::Error(crate::error::ValueError::Value),
    }
}

pub fn subtract_days_from_date(d: DateTime<FixedOffset>, rhs: Value) -> Value {
    match rhs {
        Value::Number(x) => Value::Date(d - Duration::days(x as i64)),
        _ => Value::Error(crate::error::ValueError::Value),
    }
}
