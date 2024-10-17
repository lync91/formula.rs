use crate::{error::ValueError, Value};
use chrono::{DateTime, FixedOffset};

pub fn calculate_days(date_pair: (Value, Value)) -> Value {
    let begin_of_date: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339("1900-01-01T02:00:00.000Z")
            .ok()
            .unwrap();
    let (start, end) = date_pair;
    match (start, end) {
        (Value::Date(start), Value::Date(end)) => Value::Number((end - start).num_days() as f64),
        (Value::Blank, Value::Date(end)) => Value::Number((end - begin_of_date).num_days() as f64),
        (Value::Date(start), Value::Blank) => {
            Value::Number((begin_of_date - start).num_days() as f64)
        }
        (Value::Blank, Value::Blank) => Value::Number(0.0),
        _ => Value::Error(ValueError::Value),
    }
}
