use super::*;
use crate::parser::function::Function;
use crate::parser::grammar::Rule;
use crate::parser::operators::Operator;
use crate::value::{Boolean, Value};
use crate::{Expression, Formula};

pub fn build_formula_number(pair: pest::iterators::Pair<Rule>) -> Formula {
    let x = pair.as_str().parse::<f64>().unwrap();
    let value = Value::Number(x);
    Formula::Value(value)
}

pub fn build_formula_string_double_quote(pair: pest::iterators::Pair<Rule>) -> Formula {
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = Value::Text(string.replace("\"\"", "\""));
    Formula::Value(value)
}

pub fn build_formula_string_single_quote(pair: pest::iterators::Pair<Rule>) -> Formula {
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = Value::Text(string);
    Formula::Value(value)
}

pub fn build_formula_boolean(boolean_value: bool) -> Formula {
    if boolean_value {
        Formula::Value(Value::Boolean(Boolean::True))
    } else {
        Formula::Value(Value::Boolean(Boolean::False))
    }
}

pub fn build_formula_iterator(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(super::pratt_parser::build_formula_with_pratt(
            term.into_inner(),
            f,
        ));
    }
    Formula::Iterator(vec)
}

pub fn build_formula_iff(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(&str, Vec<f64>) -> Value>,
) -> Formula {
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(Formula::Value(Value::Blank))
        } else {
            vec.push(build_formula_with_pratt(term.into_inner(), f))
        }
    }
    let operation = Expression {
        op: Operator::Function(Function::Iff),
        values: vec,
    };
    Formula::Operation(operation)
}
