use serde_derive::{Deserialize, Serialize};

use crate::value::{value_type::ValueType, Value};

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum EvalError {
    ContextNotMutable,
    ExpectedNumber {
        /// The actual value.
        actual: Value,
    },
    ExpectedString {
        /// The actual value.
        actual: Value,
    },
    ExpectedBoolean {
        /// The actual value.
        actual: Value,
    },
    ExpectedInt {
        /// The actual value.
        actual: Value,
    },
    ExpectedFloat {
        /// The actual value.
        actual: Value,
    },
    ExpectedEmpty {
        /// The actual value.
        actual: Value,
    },
}

impl EvalError {
    pub fn expected_number(actual: Value) -> Self {
        Self::ExpectedNumber { actual }
    }

    pub fn expected_string(actual: Value) -> Self {
        Self::ExpectedString { actual }
    }

    pub fn expected_boolean(actual: Value) -> Self {
        Self::ExpectedBoolean { actual }
    }

    pub fn expected_int(actual: Value) -> Self {
        Self::ExpectedInt { actual }
    }

    pub fn expected_float(actual: Value) -> Self {
        Self::ExpectedFloat { actual }
    }

    pub(crate) fn expected_type(expected: &Value, actual: Value) -> Self {
        match ValueType::from(expected) {
            ValueType::String => Self::expected_string(actual),
            ValueType::Int => Self::expected_int(actual),
            ValueType::Float => Self::expected_float(actual),
            ValueType::Boolean => Self::expected_boolean(actual),
            // ValueType::Tuple => Self::expected_tuple(actual),
            ValueType::Empty => Self::expected_empty(actual),
        }
    }
    /// Constructs `EvalexprError::ExpectedEmpty{actual}`.
    pub fn expected_empty(actual: Value) -> Self {
        EvalError::ExpectedEmpty { actual }
    }
}

// impl EvalError {
//     pub(crate) fn expected_type(expected: &Value, actual: Value) -> Self {
//         match ValueType::from(expected) {
//             ValueType::String => Self::expected_string(actual),
//             ValueType::Int => Self::expected_int(actual),
//             ValueType::Float => Self::expected_float(actual),
//             ValueType::Boolean => Self::expected_boolean(actual),
//             ValueType::Tuple => Self::expected_tuple(actual),
//             ValueType::Empty => Self::expected_empty(actual),
//         }
//     }
// }

/// Defines error types.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueError {
    Div0,
    Cast,
    Parse,
    Value,
    Argument,
    Reference,
}
