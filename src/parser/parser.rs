use super::grammar::{Grammar, Rule};
use crate::error::ValueError;
use crate::parser::builder::build_formula_with_pratt;
use crate::value::Value;
use crate::Formula;
use pest::Parser; // Gọi các hàm build từ module builder

pub fn parse_string(s: &str) -> Option<pest::iterators::Pair<Rule>> {
    let parse_result = Grammar::parse(Rule::formula, s);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            Some(parse_result)
        }
        Err(_) => None,
    }
}

fn parse_string_constant(parse_result: pest::iterators::Pair<Rule>) -> Formula {
    let string = parse_result
        .into_inner()
        .as_str()
        .parse::<String>()
        .unwrap();
    Formula::Value(Value::Text(string.trim_start_matches('\'').to_string()))
}

pub fn parse_string_to_formula(s: &str, f: Option<&impl Fn(&str, Vec<f64>) -> Value>) -> Formula {
    match parse_string(&s) {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_pratt(parse_result.into_inner(), f),
            Rule::string_constant => parse_string_constant(parse_result),
            _ => Formula::Value(Value::Error(ValueError::Parse)),
        },
        None => Formula::Value(Value::Error(ValueError::Parse)),
    }
}
