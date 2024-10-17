use super::calculator::calculate_formula;
use crate::{value::Value, Formula};

pub fn calculate_iterator(mut vec: Vec<Formula>, f: Option<&impl Fn(&str) -> Value>) -> Value {
    let mut value_vec = Vec::new();
    while let Some(top) = vec.pop() {
        value_vec.push(calculate_formula(top, f));
    }
    Value::Iterator(value_vec)
}
