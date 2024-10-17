use std::collections::HashMap;

use crate::{
    error::{EvalError, EvalResult},
    function::Function,
    value::{value_type::ValueType, Value},
};

pub trait Context {
    /// Returns the value that is linked to the given identifier.
    fn get_value(&self, identifier: &str) -> Option<&Value>;

    /// Calls the function that is linked to the given identifier with the given argument.
    /// If no function with the given identifier is found, this method returns `EvalexprError::FunctionIdentifierNotFound`.
    // fn call_ function(&self, identifier: &str, argument: &Value) -> EvalexprResult<Value>;

    /// Checks if builtin functions are disabled.
    fn are_builtin_functions_disabled(&self) -> bool;

    // Disables builtin functions if `disabled` is `true`, and enables them otherwise.
    // If the context does not support enabling or disabling builtin functions, an error is returned.
    // fn set_builtin_functions_disabled(&mut self, disabled: bool) -> EvalexprResult<()>;
}

/// A context that allows to assign to variables.
pub trait ContextWithMutableVariables: Context {
    /// Sets the variable with the given identifier to the given value.
    fn set_value(&mut self, _identifier: String, _value: Value) -> EvalResult<()> {
        Err(EvalError::ContextNotMutable)
    }
}

#[derive(Clone, Debug, Default)]
pub struct HashMapContext {
    variables: HashMap<String, Value>,
    // functions: HashMap<String, Function>,
    /// True if builtin functions are disabled.
    without_builtin_functions: bool,
}

impl Context for HashMapContext {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        self.variables.get(identifier)
    }

    // fn call_function(&self, identifier: &str, argument: &Value) -> EvalexprResult<Value> {
    //     if let Some(function) = self.functions.get(identifier) {
    //         function.call(argument)
    //     } else {
    //         Err(EvalexprError::FunctionIdentifierNotFound(
    //             identifier.to_string(),
    //         ))
    //     }
    // }

    fn are_builtin_functions_disabled(&self) -> bool {
        self.without_builtin_functions
    }

    // fn set_builtin_functions_disabled(&mut self, disabled: bool) -> EvalexprResult<()> {
    //     self.without_builtin_functions = disabled;
    //     Ok(())
    // }
}

impl ContextWithMutableVariables for HashMapContext {
    fn set_value(&mut self, identifier: String, value: Value) -> EvalResult<()> {
        if let Some(existing_value) = self.variables.get_mut(&identifier) {
            if ValueType::from(&existing_value) == ValueType::from(&value) {
                *existing_value = value;
                return Ok(());
            } else {
                return Err(EvalError::expected_type(existing_value, value));
            }
        }

        // Implicit else, because `self.variables` and `identifier` are not unborrowed in else
        self.variables.insert(identifier, value);
        Ok(())
    }
}

impl HashMapContext {
    pub fn new() -> Self {
        Default::default()
    }
}
