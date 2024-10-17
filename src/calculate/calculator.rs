use super::iterator::calculate_iterator;
use super::operators::calculate_operation;
use super::reference::calculate_reference;
use crate::value::Value;
use crate::Formula;
/// Evaluates a string that was parsed and stored in Expression Struct.
/// Takes an optional closure with the trait bound Fn(String) -> types::Value.
pub fn calculate_formula(formula: Formula, f: Option<&impl Fn(&str) -> Value>) -> Value {
    match formula {
        Formula::Operation(exp) => calculate_operation(exp, f),
        Formula::Value(val) => val,
        Formula::Reference(r) => calculate_reference(r.as_str(), f),
        Formula::Iterator(vec) => calculate_iterator(vec, f),
    }
}

#[cfg(test)]

mod tests {}
