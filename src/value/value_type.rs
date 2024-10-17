use super::*;

/// The type of a `Value`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ValueType {
    /// The `Value::String` type.
    String,
    /// The `Value::Float` type.
    Float,
    /// The `Value::Int` type.
    Int,
    /// The `Value::Boolean` type.
    Boolean,
    /// The `Value::Tuple` type.
    // Tuple,
    /// The `Value::Empty` type.
    Empty,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::Text(_) => ValueType::String,
            Value::Number(_) => ValueType::Float,
            // Value::Number(_) => ValueType::Int,
            Value::Boolean(_) => ValueType::Boolean,
            // Value::Tuple(_) => ValueType::Tuple,
            Value::Blank => ValueType::Empty,
            _ => ValueType::Empty,
        }
    }
}

impl From<&mut Value> for ValueType {
    fn from(value: &mut Value) -> Self {
        From::<&Value>::from(value)
    }
}

impl From<&&mut Value> for ValueType {
    fn from(value: &&mut Value) -> Self {
        From::<&Value>::from(*value)
    }
}
