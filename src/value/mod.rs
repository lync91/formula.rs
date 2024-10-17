use chrono::{DateTime, FixedOffset};
use serde_derive::{Deserialize, Serialize};

use crate::error::{EvalError, EvalResult, ValueError};

pub mod value_type;

/// The type used to represent integers in `Value::Int`.
pub type IntType = i64;

/// The type used to represent floats in `Value::Float`.
pub type FloatType = f64;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType = Vec<Value>;

/// The type used to represent empty values in `Value::Empty`.
pub type EmptyType = ();

/// The value of the empty type to be used in rust.
pub const EMPTY_VALUE: () = ();

/// Defines boolean types.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Boolean {
    True,
    False,
}

impl Boolean {
    /// Returns true if `self` is a `Boolean::True`.
    pub fn is_true(&self) -> bool {
        matches!(self, Boolean::True)
    }

    /// Returns true if `self` is a `Boolean::False`.
    pub fn is_false(&self) -> bool {
        matches!(self, Boolean::False)
    }

    pub fn to_string(&self) -> String {
        match self {
            Boolean::True => "TRUE".to_string(),
            Boolean::False => "FALSE".to_string(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Value {
    Number(f64),
    Text(String),
    Boolean(Boolean),
    Iterator(Vec<Value>),
    Error(ValueError),
    Date(DateTime<FixedOffset>),
    #[default]
    Blank,
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::Text(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    // pub fn is_int(&self) -> bool {
    //     matches!(self, Value::Boolean(_))
    // }

    /// Returns true if `self` is a `Value::Float`.
    // pub fn is_float(&self) -> bool {
    //     matches!(self, Value::Number(_))
    // }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    // pub fn is_tuple(&self) -> bool {
    //     matches!(self, Value::Tuple(_))
    // }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_blank(&self) -> bool {
        matches!(self, Value::Blank)
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_text(&self) -> EvalResult<String> {
        match self {
            Value::Text(string) => Ok(string.clone()),
            value => Err(EvalError::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    // pub fn as_int(&self) -> EvalResult<IntType> {
    //     match self {
    //         Value::Int(i) => Ok(*i),
    //         value => Err(EvalError::expected_int(value.clone())),
    //     }
    // }

    // /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    // pub fn as_float(&self) -> EvalResult<FloatType> {
    //     match self {
    //         Value::Float(f) => Ok(*f),
    //         value => Err(EvalError::expected_float(value.clone())),
    //     }
    // }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> EvalResult<FloatType> {
        match self {
            Value::Number(f) => Ok(*f),
            // Value::Int(i) => Ok(*i as FloatType),
            value => Err(EvalError::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> EvalResult<bool> {
        match self {
            Value::Boolean(boolean) => Ok(boolean.to_bool()),
            value => Err(EvalError::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType`, or returns `Err` if `self` is not a `Value::Tuple`.
    // pub fn as_tuple(&self) -> EvalResult<TupleType> {
    //     match self {
    //         Value::Tuple(tuple) => Ok(tuple.clone()),
    //         value => Err(EvalError::expected_tuple(value.clone())),
    //     }
    // }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    // pub fn as_fixed_len_tuple(&self, len: usize) -> EvalResult<TupleType> {
    //     match self {
    //         Value::Tuple(tuple) => {
    //             if tuple.len() == len {
    //                 Ok(tuple.clone())
    //             } else {
    //                 Err(EvalError::expected_fixed_len_tuple(len, self.clone()))
    //             }
    //         }
    //         value => Err(EvalError::expected_tuple(value.clone())),
    //     }
    // }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` with length in the required range.
    // pub fn as_ranged_len_tuple(&self, range: RangeInclusive<usize>) -> EvalResult<TupleType> {
    //     match self {
    //         Value::Tuple(tuple) => {
    //             if range.contains(&tuple.len()) {
    //                 Ok(tuple.clone())
    //             } else {
    //                 Err(EvalError::expected_ranged_len_tuple(
    //                     range,
    //                     self.clone(),
    //                 ))
    //             }
    //         }
    //         value => Err(EvalError::expected_tuple(value.clone())),
    //     }
    // }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    // pub fn as_empty(&self) -> EvalResult<()> {
    //     match self {
    //         Value::Empty => Ok(()),
    //         value => Err(EvalError::expected_empty(value.clone())),
    //     }
    // }

    /// Returns a string for the `str::from` built-in function.
    pub fn str_from(&self) -> String {
        match self {
            Value::Text(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            // Value::Int(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            // Value::Tuple(_) => self.to_string(),
            Value::Blank => String::from(""),
            Value::Error(_) => String::from("#ERROR"),
            Value::Iterator(vs) => {
                let mut s = String::from("[");
                for v in vs {
                    s.push_str(&v.str_from());
                    s.push_str(", ");
                }
                s.push_str("]");
                s
            }
            Value::Date(d) => d.to_string(),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::Text(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::Text(string.to_string())
    }
}

impl From<f64> for Value {
    fn from(float: FloatType) -> Self {
        Value::Number(float)
    }
}

impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Value::Number(int as f64)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        match boolean {
            true => Value::Boolean(Boolean::True),
            false => Value::Boolean(Boolean::False),
        }
    }
}

// impl From<TupleType> for Value {
//     fn from(tuple: TupleType) -> Self {
//         Value::Tuple(tuple)
//     }
// }

impl From<Value> for EvalResult<Value> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Blank
    }
}

impl TryFrom<Value> for String {
    type Error = EvalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Text(value) = value {
            Ok(value)
        } else {
            Err(EvalError::ExpectedString { actual: value })
        }
    }
}

impl TryFrom<Value> for FloatType {
    type Error = EvalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Number(value) = value {
            Ok(value)
        } else {
            Err(EvalError::ExpectedNumber { actual: value })
        }
    }
}

impl TryFrom<Value> for IntType {
    type Error = EvalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Number(value) = value {
            Ok(value as i64)
        } else {
            Err(EvalError::ExpectedInt { actual: value })
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = EvalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Boolean(value) = value {
            match value {
                Boolean::True => Ok(true),
                Boolean::False => Ok(false),
            }
        } else {
            Err(EvalError::ExpectedBoolean { actual: value })
        }
    }
}

// impl TryFrom<Value> for TupleType {
//     type Error = EvalexprError;

//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         if let Value::Tuple(value) = value {
//             Ok(value)
//         } else {
//             Err(EvalexprError::ExpectedTuple { actual: value })
//         }
//     }
// }

impl TryFrom<Value> for () {
    type Error = EvalError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Blank = value {
            Ok(())
        } else {
            Err(EvalError::ExpectedEmpty { actual: value })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_string() {
        let value: Value = "Hello, World!".into();
        assert_eq!(value, Value::Text("Hello, World!".to_string()));
    }

    #[test]
    fn test_value_from_float() {
        let value: Value = 3.14.into();
        assert_eq!(value, Value::Number(3.14));
    }

    #[test]
    fn test_value_from_int() {
        let value: Value = 42.into();
        assert_eq!(value, Value::Number(42.0));
    }

    #[test]
    fn test_value_from_bool() {
        let value: Value = true.into();
        assert_eq!(value, Value::Boolean(Boolean::True));

        let value: Value = false.into();
        assert_eq!(value, Value::Boolean(Boolean::False));
    }

    // #[test]
    // fn test_value_from_tuple() {
    //     let value: Value = vec![Value::Number(1.0), Value::Number(2.0)].into();
    //     assert_eq!(value, Value::Tuple(vec![Value::Number(1.0), Value::Number(2.0)]));
    // }

    #[test]
    fn test_value_from_unit() {
        let value: Value = ().into();
        assert_eq!(value, Value::Blank);
    }

    #[test]
    fn test_try_from_string() {
        let value: Value = "Hello, World!".into();
        let string: String = value.try_into().unwrap();
        assert_eq!(string, "Hello, World!");
    }

    #[test]
    fn test_try_from_float() {
        let value: Value = 3.14.into();
        let float: FloatType = value.try_into().unwrap();
        assert_eq!(float, 3.14);
    }

    #[test]
    fn test_try_from_int() {
        let value: Value = 42.into();
        let int: IntType = value.try_into().unwrap();
        assert_eq!(int, 42);
    }

    #[test]
    fn test_try_from_bool() {
        let value: Value = true.into();
        let boolean: bool = value.try_into().unwrap();
        assert_eq!(boolean, true);

        let value: Value = false.into();
        let boolean: bool = value.try_into().unwrap();
        assert_eq!(boolean, false);
    }
}
