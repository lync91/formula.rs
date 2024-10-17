use std::fmt;

use crate::{error::EvalResult, value::Value};

trait ClonableFn
where
    Self: Fn(&Value) -> EvalResult<Value>,
    Self: Send + Sync + 'static,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn>;
}

impl<F> ClonableFn for F
where
    F: Fn(&Value) -> EvalResult<Value>,
    F: Send + Sync + 'static,
    F: Clone,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn> {
        Box::new(self.clone()) as _
    }
}

pub struct Function {
    function: Box<dyn ClonableFn>,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            function: self.function.dyn_clone(),
        }
    }
}

impl Function {
    /// Creates a user-defined function.
    ///
    /// The `function` is boxed for storage.
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(&Value) -> EvalResult<Value>,
        F: Send + Sync + 'static,
        F: Clone,
    {
        Self {
            function: Box::new(function) as _,
        }
    }

    pub(crate) fn call(&self, argument: &Value) -> EvalResult<Value> {
        (self.function)(argument)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Function {{ [...] }}")
    }
}
